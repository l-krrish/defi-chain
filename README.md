# DeFi Chain: Complete Rust Project 🚀

A complete, modular Rust project for building blockchain and DeFi primitives in a single workspace.

This project includes:
- ⛓️ A proof-of-work blockchain engine
- 👛 Wallet key generation and address derivation
- 📥 A fee-prioritized mempool
- 💱 DeFi modules (AMM, lending, staking)
- 🌐 Peer networking layer
- 💾 Persistent storage wrapper
- 🛠️ RPC/CLI crate with a runnable node binary

## Workspace Layout 📦

- `blockchain/` - block, chain, and mining logic
- `wallet/` - keypair generation, address derivation, transaction type
- `mempool/` - pending transaction pool with fee ordering and TTL pruning
- `defi/amm/` - constant-product AMM simulation
- `defi/lending/` - collateralized borrowing simulation
- `defi/staking/` - token locking and reward computation
- `network/` - basic peer registry and message broadcasting/gossip behavior
- `storage/` - sled-based key-value storage wrapper
- `rpc/` - request/response types, CLI config, and executable entrypoint (`defi-chain`)

## Requirements ✅

- Rust (stable)
- Cargo

Install Rust from: https://rustup.rs

## Build 🔧

```bash
cargo build
```

## Test 🧪

```bash
cargo test
```

Current status: tests pass in this workspace (one AMM unit test).

## Run the Node CLI ▶️

The binary is defined in `rpc/src/main.rs` and named `defi-chain`.

```bash
cargo run -p rpc -- --port 3000 --difficulty 2
```

With an initial peer:

```bash
cargo run -p rpc -- --port 3000 --difficulty 3 --peer 127.0.0.1:4001
```

### CLI Options

- `--port <u16>` (default: `3000`)
- `--difficulty <usize>` (default: `2`)
- `--peer <string>` (optional)

## Module Highlights 🧱

### Blockchain ⛏️

- `Block` includes index, timestamp, previous hash, nonce, data, and hash
- `mine` increments nonce until hash has `difficulty` leading zeroes
- `Chain::new` creates and mines a genesis block
- `Chain::add_block` mines and appends a new block

### Wallet 🔐

- Uses `k256` for ECDSA key generation
- Derives addresses by Keccak-256 hashing compressed pubkey bytes and taking the last 20 bytes (`0x`-prefixed)
- Includes a `Transaction` struct with signing payload bytes helper

### Mempool 📬

- Stores txs grouped by fee in a `BTreeMap`
- `pop_batch(n)` returns highest-fee txs first
- `prune_expired(now)` removes old txs based on TTL

### DeFi 💸

- AMM: constant product style swap with 0.3% fee approximation
- Lending: collateral tracking, borrow limit (75%), health factor, liquidation path
- Staking: lock positions and compute block-based rewards

### Network 📡

- Tracks known peers
- Broadcasts messages to all peers
- Simulates gossip rebroadcast to peers except sender

### Storage 🗃️

- Wraps `sled` with `open`, `save`, and `load`

## Quick Code Example 👇

```rust
use blockchain::chain::Chain;

fn main() {
    let mut chain = Chain::new(2);
    chain.add_block("first block".to_string());
    println!("Tip hash: {}", chain.tip().hash);
}
```

## Project Overview 🏁

DeFi Chain is structured as a complete Rust workspace with dedicated crates for chain execution, wallet logic, mempool processing, DeFi protocols, networking, storage, and RPC/CLI interaction.

## License 📄

Add a `LICENSE` file to define project licensing.
