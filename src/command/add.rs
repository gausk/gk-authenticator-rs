use crate::command::Algorithm;
use crate::store::{Account, AccountStore};
use anyhow::Result;

pub fn add_authenticator_account(
    store: &mut AccountStore,
    name: String,
    key: String,
    algorithm: Algorithm,
    hotp: bool,
) -> Result<()> {
    let account = Account::new(name, key, algorithm, !hotp);
    store.add_account(account)
}
