[Rust Language]: https://rust-lang.org

# GitHub Synchronize

Synchronize GitHub Forks using the GitHub Command Line Interface

## Features

- Synchronize Forks

## Build

- [Rust][Rust Language]

```shell
git clone

cargo check

cargo build --release --target x86_64-unknown-linux-gnu

mv ./target/x86_64-unknown-linux-gnu/release/gh-sync ./binary

sudo ./binary/gh-sync
```

## Install GitHub Synchronize

```shell
sudo install ./gh-sync /usr/local/bin
```

