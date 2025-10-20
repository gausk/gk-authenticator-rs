use crate::command::add::add_authenticator_account;
use crate::command::delete::delete_authenticator_account;
use crate::command::list::list_all_authenticator;
use crate::command::view::view_particular_authenticator_account;
use crate::command::{Arg, Command};
use crate::store::AccountStore;
use anyhow::Result;
use clap::Parser;

mod command;
mod otp;
mod store;

fn main() -> Result<()> {
    let arg = Arg::try_parse()?;
    let mut store = AccountStore::load()?;
    match arg.command {
        Command::Add {
            account,
            key,
            hotp,
            totp: _,
            algorithm,
        } => add_authenticator_account(&mut store, account, key, algorithm, hotp),
        Command::Delete { account } => delete_authenticator_account(&mut store, &account),
        Command::List { length } => {
            list_all_authenticator(&mut store, length);
            Ok(())
        }
        Command::View { account, length } => {
            let otp = view_particular_authenticator_account(&mut store, &account, length)?;
            println!("{}", otp);
            Ok(())
        }
    }
}
