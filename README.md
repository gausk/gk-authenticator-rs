# gk-authenticator

A **Command Line OTP authenticator app** written in Rust that generates time and counter based OTP codes.

## Installation

`gk-authenticator` is built with Rust and is available via the **Cargo** package manager.

### Prerequisites

You must have the [Rust toolchain](https://www.rust-lang.org/tools/install) installed to use Cargo.

### Install via Cargo

Run the following command in your terminal:

```bash
cargo install gk-authenticator
```

### Features
1. Supports both Time-based OTP (TOTP) and Counter-based OTP (HOTP). 
2. Allows selection of various hash algorithms: SHA1 (default), SHA256, SHA384, and SHA512. 
3. All secret keys must be provided in Base32 encoding.
4. Simple command-line interface for managing accounts.

### Usage

Once installed, you can use `gk-authenticator` with the following commands:

#### Add an Account
Add a new authenticator account with a secret key:

```bash
# Add a TOTP account (default)
gk-authenticator add --account "GitHub" --key "JBSWY3DPEHPK3PXP"

# Add an HOTP account
gk-authenticator add --account "MyService" --key "JBSWY3DPEHPK3PXP" --hotp

# Add with custom algorithm
gk-authenticator add --account "MyApp" --key "JBSWY3DPEHPK3PXP" --algorithm sha256
```

**Options:**
- `--account`: Name of the account (required)
- `--key`, `-k`: Secret key in Base32 encoding (required)
- `--totp`: Use Time-based OTP (default, conflicts with --hotp)
- `--hotp`: Use Counter-based OTP (conflicts with --totp)
- `--algorithm`, `-a`: Hash algorithm (sha1, sha256, sha384, sha512) - default: sha1

#### View OTP for a Specific Account
Generate and display the OTP code for a particular account:

```bash
# View OTP with default 6-digit length
gk-authenticator view --account "GitHub"

# View OTP with custom length
gk-authenticator view --account "GitHub" --length 8
```

**Options:**
- `--account`: Name of the account (required)
- `--length`, `-l`: Length of the OTP code (default: 6)

#### List All OTPs
Display OTP codes for all stored accounts:

```bash
# List all with default 6-digit length
gk-authenticator list

# List all with custom length
gk-authenticator list --length 8
```

**Options:**
- `--length`, `-l`: Length of the OTP code (default: 6)

#### Delete an Account
Remove an account from storage:

```bash
gk-authenticator delete --account "GitHub"
```

**Options:**
- `--account`: Name of the account to delete (required)

#### Help
Get help for any command:

```bash
# General help
gk-authenticator --help

# Help for specific command
gk-authenticator add --help
```