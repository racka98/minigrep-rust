# MiniGrep

Project from Rust Book which makes a light weight version of the `grep` CLI tool

## Usage

- In this project dir & with `rustc` installed on your machine (see #Build section)

```sh
# Case Sensitive (ignores letter case)
cargo run -- word file.extension

# Case Insensitve
## 1. In sh & zsh
IGNORE_CASE=1 cargo run -- word file.extension

## 2. In Windows Powershell
$Env:IGNORE_CASE=1; cargo run -- word file.extension
```

eg. `cargo run -- to poem.txt`

- With the built binary (eg. minigrep.exe)

```sh
# Case Sensitive (ignore letter case)
## In Window Powershell, sh & zsh
./mingrep word file.extension

# Case Insenstive
## 1. sh & zsh
IGNORE_CASE=1 ./minigrep word file.extension

## 2. Windows Powershell
$Env:IGNORE_CASE=1; ./minigrep word file.extension

## 3. Remove IGNORE_CASE environment variable in Windows Powershell
Remove-Item Env:IGNORE_CASE
```

## Build Binary

1. Install Rust by following [the official docs](https://www.rust-lang.org/tools/install)

2. Run `cargo build` for Debug build or `cargo build --release` Release build

3. You'll find your binaries in `target/debug` & `target/release` for Debug & Release respectively

## Tests

Run:

```sh
cargo test

## With standard output data
cargo test -- --show-output
```
