# Ethereum Explorer

Ethereum Explorer is a Rust application that allows you to fetch information about Ethereum blocks through a graphical user interface (GUI) and an API.

## Features

- Fetch information about Ethereum blocks using block numbers.
- GUI interface for user-friendly interaction.
- API endpoint for programmatic access to block information.

## Dependencies

- [actix-web](https://crates.io/crates/actix-web): A powerful and ergonomic web framework for Rust.
- [async-trait](https://crates.io/crates/async-trait): Provides procedural macros for defining async version of traits.
- [gtk](https://crates.io/crates/gtk): Rust bindings to the GTK+ library for creating GUI applications.
- [tokio](https://crates.io/crates/tokio): An asynchronous runtime for Rust.
- [web3](https://crates.io/crates/web3): Ethereum JSON-RPC library for Rust.

## Prerequisites

- Rust: Make sure you have Rust installed. You can install it from the official website: https://www.rust-lang.org/

## Installation and Usage

1. Clone the repository:

   ```bash
   git clone https://github.com/your-username/ethereum-explorer.git


### Navigate Project Directory

```shell script
cd ethereum-explorer
```


### Build and run the project

```shell script
cargo run
```