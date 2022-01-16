# rust-training

This repository is used to practice the Rust programming language and associated tools.

## Installing Rust (macOS)

Use:

```
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

This also installs `cargo` and `rustup`.

### Patch / Update

Use:

```
$ rustup update
```


## Create new binary} project

Leverage `cargo` for this:

```
$ cargo new <name> --bin
```

which will give:

```
.
├── Cargo.toml
└── src
    └── main.rs
```
