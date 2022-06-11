# Tashikani Bot

[![Rust Version](https://img.shields.io/badge/1.58.0+-f74b00.svg?&label=rust&logo=rust)](https://github.com/rust-lang/rust)

A Poketwo-like bot spinoff for kanji learning written in Rust using Serenity.

## Requirements

- Rust 1.58.0 or later
- Discord bot API key (for `TOKEN`)
- Docker (to build using Docker)
- Docker-compose (to build using Docker)

## Build

### Setting up tokens

To run the shell commands below, replace `<token>` with your API key.

```sh
# POSIX (Linux, macOS, BSD)
export TOKEN="<token>"

# Windows CMD
set TOKEN="<token>"

# Windows PowerShell
$env:TOKEN="<token>"
```

### Building the bot

#### Docker

**This command build (then run) the Discord bot container(s):**

```sh
docker-compose up --build
```

**Once built, you can run the containers again using:**

```sh
docker-compose -d
```

#### Cargo Run

**Run the bot locally in your machine:**

```sh
cargo run
```

## Test

Make sure you run this command before contributing to the main branch:

```sh
cargo fmt -- --check && cargo clippy -- -Dwarnings && cargo test
```

## Available Bot Commands

_To be updated_
