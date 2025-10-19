#![allow(unused)]
mod command;

use crate::command::Command;
use anyhow::Result;
use clap::Parser;
use command::Arg;

fn main() -> Result<()> {
    let arg = Arg::try_parse()?;
    match arg.command {
        Command::Add {
            account,
            key,
            hotp,
            totp,
            algorithm,
        } => {}
        Command::Delete { account } => {}
        Command::List { length } => {}
        Command::View { account, length } => {}
    }
    Ok(())
}
