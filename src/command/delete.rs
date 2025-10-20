use crate::store::AccountStore;
use anyhow::Result;

pub fn delete_authenticator_account(store: &mut AccountStore, account_name: &str) -> Result<()> {
    store.delete(account_name)
}
