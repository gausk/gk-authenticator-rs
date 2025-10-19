#![allow(unused)]
mod command;

use anyhow::Result;
use clap::Parser;
use command::Arg;
use crate::command::Command;

fn main() -> Result<()> {
    let arg = Arg::try_parse()?;
    match arg.command {
        Command::Add { account, key, htop, totp, algorithm } => {

        }
        Command::Delete { account} => {

        }
        Command::List { length} => {

        }
        Command::View { account, length } => {

        }
    }
    Ok(())
}
