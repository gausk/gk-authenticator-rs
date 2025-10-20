use crate::otp::Otp;
use crate::store::AccountStore;
use anyhow::Result;

pub fn view_particular_authenticator_account(
    store: &mut AccountStore,
    account_name: &str,
    length: usize,
) -> Result<String> {
    let account = store.get_account_mut(account_name)?;
    let otp = Otp::new(
        &account.key,
        account.algorithm,
        account.totp,
        account.counter,
        length,
    )?;
    let out = otp.generate();
    if let Some(counter) = account.counter.as_mut() {
        *counter += 1;
    }
    Ok(out)
}
