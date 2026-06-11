# Contracts

The onchain pillar of `heliosproto`. All contracts are written in Rust against the [Soroban SDK](https://docs.rs/soroban-sdk).

See the [Pillar 1 section of ARCHITECTURE.md](../ARCHITECTURE.md#3-pillar-1--contracts-rust--soroban-sdk) for the full design.

## Layout

| Path | Purpose |
|---|---|
| `smart-account/` | Core account contract: signers, nonce, plugin set, `__check_auth` |
| `plugins/session-keys/` | Time-bounded scoped signing keys |
| `plugins/social-recovery/` | M-of-N guardian recovery with timelock |
| `plugins/spending-limits/` | Per-token, per-period spending caps |
| `plugins/time-lock/` | Mandatory delay for high-value actions |
| `plugins/allowlist/` | Restrict outbound calls to whitelisted contracts |
| `paymaster/` | Sponsored transaction relayer contract |
| `factory/` | Deterministic account deployment via passkey-derived salt |

## Prerequisites

- Rust 1.84.0+ with `wasm32-unknown-unknown` target
- Stellar CLI (`cargo install --locked stellar-cli --features opt`)

## Working in this pillar

Each contract is a standalone Cargo crate. From inside any contract directory:

```bash
cargo test                  # unit tests against the local sandbox
cargo build --target wasm32-unknown-unknown --release
stellar contract optimize --wasm target/wasm32-unknown-unknown/release/<name>.wasm
```

A workspace `Cargo.toml` at the pillar root will be added when the first contract crate lands.
