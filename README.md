# PPV Subscription Contract — Soroban (Stellar)

A **Pay-per-View (PPV) content access** smart contract built on the Stellar blockchain using the Soroban smart contract platform. Content owners can register payable content, and viewers can subscribe using XLM to gain time-limited access.

---

## Features

- Register content with a custom price (in XLM)
- Subscribe to content for a defined time period (measured in ledgers)
- Check whether a subscriber currently has active access
- Update content price at any time (owner only)
- Renew or cancel subscriptions
- Deactivate content from the platform (owner only)

---

## Project Structure

```
ppv-subscription/
├── src/
│   ├── lib.rs       # Main smart contract (CRUD logic)
│   └── test.rs      # Unit tests
├── Cargo.toml       # Dependencies and build config
└── README.md        # This file
```

---

## CRUD Operations

| Operation | Function               | Caller         | Description                              |
|-----------|------------------------|----------------|------------------------------------------|
| CREATE    | `add_content`          | Content owner  | Register new content with a price        |
| CREATE    | `subscribe`            | Viewer         | Subscribe to content for N ledgers       |
| READ      | `get_content`          | Anyone         | Fetch content metadata by ID             |
| READ      | `get_subscription`     | Anyone         | Fetch subscription details               |
| READ      | `check_access`         | Anyone         | Returns true if subscription is active   |
| UPDATE    | `update_price`         | Content owner  | Change the price of existing content     |
| UPDATE    | `renew_subscription`   | Viewer         | Extend an existing subscription          |
| DELETE    | `remove_content`       | Content owner  | Deactivate content from the platform     |
| DELETE    | `cancel_subscription`  | Viewer         | Cancel an active subscription            |

---

## Data Structures

### Content
```rust
pub struct Content {
    pub content_id: u64,
    pub title:      String,
    pub owner:      Address,
    pub price:      i128,   // in stroops (1 XLM = 10_000_000 stroops)
    pub is_active:  bool,
}
```

### Subscription
```rust
pub struct Subscription {
    pub subscriber: Address,
    pub content_id: u64,
    pub expires_at: u64,    // ledger sequence number when access expires
    pub is_active:  bool,
}
```

---

## Price Units

Prices are stored in **stroops**, the smallest unit of XLM.

| XLM  | Stroops       |
|------|---------------|
| 0.1  | 1,000,000     |
| 0.5  | 5,000,000     |
| 1.0  | 10,000,000    |
| 5.0  | 50,000,000    |

---

## Ledger Duration Reference

Each Stellar ledger closes approximately every **5 seconds**.

| Duration  | Ledgers   |
|-----------|-----------|
| 1 hour    | 720       |
| 1 day     | 17,280    |
| 1 week    | 120,960   |
| 1 month   | 518,400   |

---

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.78.0 or later)
- Soroban CLI
- Stellar testnet account with XLM

Install the Soroban CLI:
```bash
cargo install --locked stellar-cli --features opt
```

Add the WebAssembly target:
```bash
rustup target add wasm32-unknown-unknown
```

---

## Build

```bash
cargo build --target wasm32-unknown-unknown --release
```

The compiled `.wasm` file will be at:
```
target/wasm32-unknown-unknown/release/ppv_subscription.wasm
```

---

## Run Tests

```bash
cargo test
```

---

## Deploy to Testnet

```bash
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/ppv_subscription.wasm \
  --source YOUR_SECRET_KEY \
  --network testnet
```

This returns a **Contract ID** — save it for invoking functions.

---

## Invoke Functions

### Add content
```bash
stellar contract invoke \
  --id CONTRACT_ID \
  --source YOUR_SECRET_KEY \
  --network testnet \
  -- add_content \
  --title "Intro to Soroban" \
  --owner YOUR_ADDRESS \
  --price 5000000
```

### Subscribe
```bash
stellar contract invoke \
  --id CONTRACT_ID \
  --source VIEWER_SECRET_KEY \
  --network testnet \
  -- subscribe \
  --subscriber VIEWER_ADDRESS \
  --content_id 1 \
  --duration 17280
```

### Check access
```bash
stellar contract invoke \
  --id CONTRACT_ID \
  --network testnet \
  -- check_access \
  --subscriber VIEWER_ADDRESS \
  --content_id 1
```

### Update price
```bash
stellar contract invoke \
  --id CONTRACT_ID \
  --source YOUR_SECRET_KEY \
  --network testnet \
  -- update_price \
  --owner YOUR_ADDRESS \
  --content_id 1 \
  --price 8000000
```

### Cancel subscription
```bash
stellar contract invoke \
  --id CONTRACT_ID \
  --source VIEWER_SECRET_KEY \
  --network testnet \
  -- cancel_subscription \
  --subscriber VIEWER_ADDRESS \
  --content_id 1
```

---

## Security Notes

- All write operations call `require_auth()` — only the authorized address can execute them.
- Content ownership is verified before price updates or content removal.
- Subscriptions are time-locked using on-chain ledger sequence numbers — no off-chain dependency.
- Expired subscriptions return `false` on `check_access` without needing explicit deletion.

---
