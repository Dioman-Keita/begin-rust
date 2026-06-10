# begin-rust

`begin-rust` is a small Rust learning workspace containing simple command-line projects.

## Projects

- `begin-rust/`: an introductory Rust example
- `todo-cli/`: a task manager CLI that lets you add and list tasks

Each project is an independent Rust crate with its own `Cargo.toml` file.

## Requirements

- Rust toolchain installed via [rustup](https://rustup.rs/)

## How to run

Change into the project directory you want to run, then use `cargo run`.

For example:

```bash
cd todo-cli
cargo run
```

You can also build a project without running it:

```bash
cargo build
```

## Repository layout

```text
begin-rust/
├── begin-rust/
│   ├── Cargo.toml
│   └── src/
├── todo-cli/
│   ├── Cargo.toml
│   └── src/
└── README.md
```

## Notes

The projects in this repository are intentionally simple and are meant for practicing Rust basics.