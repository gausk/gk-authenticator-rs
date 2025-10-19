#![allow(unused)]
mod command;

use clap::Parser;
use command::Arg;
use crate::command::Command;

fn main() {
    let arg = Arg::parse();
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
}
