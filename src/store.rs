use crate::command::Algorithm;
use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::os::unix::fs::OpenOptionsExt;
use std::path::PathBuf;
use std::sync::LazyLock;
use std::{env, fs};

const AUTHENTICATOR_DID_NAME: &str = ".gk-authenticator";
const ACCOUNT_STORE_FILE_NAME: &str = "account.json";

static ACCOUNT_STORE_FILE_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    get_account_store_path()
        .inspect_err(|e| eprintln!("{e}"))
        .expect("error reading account store file")
});

fn get_account_store_path() -> Result<PathBuf> {
    let home_dir = env::home_dir().with_context(|| "Home directory not found")?;
    let authenticator_dir = home_dir.join(AUTHENTICATOR_DID_NAME);
    fs::create_dir_all(&authenticator_dir)
        .with_context(|| format!("Failed to create authenticator dir {:?}", authenticator_dir))?;
    let account_store_file = authenticator_dir.join(ACCOUNT_STORE_FILE_NAME);
    if !account_store_file.is_file() {
        fs::OpenOptions::new()
            .mode(0o600)
            .write(true)
            .create_new(true)
            .open(&account_store_file)
            .with_context(|| format!("Failed to open or create file: {:?}", account_store_file))?;
    }
    Ok(account_store_file)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub(crate) name: String,
    pub(crate) key: String,
    pub(crate) algorithm: Algorithm,
    pub(crate) totp: bool,
    pub(crate) counter: Option<u64>,
}

impl Account {
    pub fn new(name: String, key: String, algorithm: Algorithm, totp: bool) -> Self {
        Self {
            name,
            key,
            algorithm,
            totp,
            counter: if !totp { Some(0) } else { None },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountStore {
    accounts: BTreeMap<String, Account>,
}

impl AccountStore {
    pub fn load() -> Result<Self> {
        let filename = &ACCOUNT_STORE_FILE_PATH;
        let data =
            fs::read_to_string(&**filename).with_context(|| "failed to read account store file")?;
        if data.trim().is_empty() {
            Ok(AccountStore {
                accounts: BTreeMap::new(),
            })
        } else {
            serde_json::from_str(&data).with_context(|| "failed to decode account store file")
        }
    }

    pub fn save(&self) -> Result<()> {
        let filename = &ACCOUNT_STORE_FILE_PATH;
        fs::write(
            &**filename,
            serde_json::to_vec(self).with_context(|| "failed to serialize account store")?,
        )
        .with_context(|| "failed to save account store")
    }

    pub fn delete(&mut self, name: &str) -> Result<()> {
        self.accounts
            .remove(name)
            .with_context(|| format!("account {} not found", name))?;
        Ok(())
    }

    fn exists(&self, name: &str) -> bool {
        self.accounts.contains_key(name)
    }

    pub fn add_account(&mut self, account: Account) -> Result<()> {
        if self.exists(account.name.as_str()) {
            bail!("account {} already exists", account.name);
        }
        self.accounts.insert(account.name.clone(), account);
        Ok(())
    }

    pub fn get_account_mut(&mut self, name: &str) -> Result<&mut Account> {
        self.accounts
            .get_mut(name)
            .with_context(|| format!("account {} does not exist", name))
    }

    pub fn mut_list(&mut self) -> &mut BTreeMap<String, Account> {
        &mut self.accounts
    }
}

impl Drop for AccountStore {
    fn drop(&mut self) {
        if let Err(e) = self.save() {
            eprintln!("Warning: {e}");
        }
    }
}
