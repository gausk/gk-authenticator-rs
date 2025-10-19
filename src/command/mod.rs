use clap::{Parser, Subcommand, ValueEnum};
use data_encoding::BASE32_NOPAD;

#[derive(Parser, Debug)]
pub struct Arg {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    #[clap(about = "Add an account")]
    Add {
        #[arg(long, help = "Name of the account")]
        account: String,
        #[arg(
            short,
            long,
            help = "Secret key of the OTP",
            value_parser(is_base32_key)
        )]
        key: String,
        #[arg(long, help = "Time based account (default)", conflicts_with = "hotp")]
        totp: bool,
        #[arg(long, help = "Counter based account", conflicts_with = "totp")]
        hotp: bool,
        #[arg(
            short,
            long,
            help = "Algorithm to use to generate the OTP code",
            default_value_t = Algorithm::Sha1,
            value_enum
        )]
        algorithm: Algorithm,
    },
    #[clap(about = "Delete an account")]
    Delete {
        #[clap(long, help = "Name of the account")]
        account: String,
    },
    #[clap(about = "List OTP for all accounts")]
    List {
        #[arg(short, long, help = "Length of the OTP", default_value_t = 6)]
        length: u64,
    },
    #[clap(about = "List OTP for a particular account")]
    View {
        #[arg(long, help = "Name of the account")]
        account: String,
        #[arg(short, long, help = "Length of the OTP", default_value_t = 6)]
        length: u64,
    },
}

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum Algorithm {
    Sha1,
    Sha256,
    Sha384,
    Sha512,
}

// Validate key provided has base32 encoding
fn is_base32_key(key: &str) -> Result<String, String> {
    let key = key.to_uppercase();
    match BASE32_NOPAD.decode(key.as_bytes()) {
        Ok(_) => Ok(key),
        Err(e) => Err(format!("Base32 decode error: {}", e)),
    }
}
