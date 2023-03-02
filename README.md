# Phat Contract Examples

This repo provides examples that demonstrate Phala phat contract features.


## Environment Preparation

An operating system of macOS or Linux systems like Ubuntu 18.04/20.04 is recommended for the workshop.
- For macOS users, we recommend to use the *Homebrew* package manager to install the dependencies
- For other Linux distribution users, use the package manager with the system like Apt/Yum

The following toolchains are needed:

- Rust toolchain
    - Install rustup, rustup is the "package manager" of different versions of Rust compilers: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
    - This will install `rustup` and `cargo`
- Ink! Contract toolchain
    - Install contract toolchain: `cargo install cargo-contract --force`
- Install frontend toolchain
    - Node.js (>=v16), follow the [official tutorial](https://nodejs.org/en/download/package-manager/)
    - Yarn (v1): `npm install --global yarn`
- Install toml-cli
    - `cargo install toml-cli`

Check your installation with

```bash
$ rustup toolchain list
# stable-x86_64-unknown-linux-gnu (default)

$ cargo --version
# cargo 1.67.1 (8ecd4f20a 2023-01-10)

$ cargo-contract --version
# cargo-contract 2.0.0-unknown-x86_64-unknown-linux-gnu

$ node --version
# v17.5.0

$ yarn --version
# 1.22.17
```

## Build

Run make in the root directory to build all the examples:

```bash
git clone https://github.com/Phala-Network/phat-contract-examples.git
make
ls dist/
```
