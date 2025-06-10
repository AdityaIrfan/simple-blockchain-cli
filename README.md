# 🧱 Simple Blockchain CLI in Rust

A simple command-line blockchain built in Rust. This project is designed for beginners to understand how blockchains work — including hashing, chaining, validation, and file-based persistence.

---

## ✨ Features

- Add new blocks with arbitrary data
- Print the blockchain
- Validate blockchain integrity
- Persist blockchain data in a local `blockchain.json` file
- Simple CLI using [`clap`](https://docs.rs/clap)

---

## 📦 Prerequisites

- [Rust (stable)](https://rustup.rs)
- Git (for cloning the repo)

To install Rust:
  ```console
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

## 🚀 Getting Started
1. Clone the repository
    ```console
    git clone https://github.com/yourusername/rust-blockchain-cli.git
    cd rust-blockchain-cli
    ```
2. Build and run
    ```console
    cargo build
    cargo run -- help
    ```

## 🛠 Available Commands
- ➕ Add a Block
    ```console
    cargo run -- add "This is my first block!"
    ```
- 📜 Print the Blockchain
    ```console
    cargo run -- print
    ```
- ✅ Validate the Blockchain
    ```console
    cargo run -- validate
    ```

## 💾 Data Persistence
- Blockchain data is stored in a file called blockchain.json.
- This file is automatically created and updated each time you add a block.
- When the CLI starts, it loads data from blockchain.json (if available).

## 📁 Project Structure
  ```bash
  src/
  ├── main.rs         # CLI and entry point
  ├── block.rs        # Block struct and hashing
  ├── blockchain.rs   # Blockchain logic (add, validate, print)
  └── storage.rs      # Persistence layer (load/save JSON)
  ```
