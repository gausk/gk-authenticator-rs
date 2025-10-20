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