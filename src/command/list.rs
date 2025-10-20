use crate::otp::Otp;
use crate::store::AccountStore;

pub fn list_all_authenticator(store: &mut AccountStore, length: usize) {
    for (name, account) in store.mut_list() {
        match Otp::new(
            &account.key,
            account.algorithm,
            account.totp,
            account.counter,
            length,
        )
        .map(|otp| otp.generate())
        {
            Ok(out) => {
                println!("{}: {}", name, out);
                if let Some(counter) = account.counter.as_mut() {
                    *counter += 1;
                }
            }
            Err(e) => {
                eprintln!("{}: {}", name, e);
            }
        }
    }
}
