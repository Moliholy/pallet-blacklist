# Blacklist pallet

[![Rust check](https://github.com/Moliholy/pallet-blacklist/actions/workflows/rust.yml/badge.svg)](https://github.com/Moliholy/pallet-blacklist/actions/workflows/rust.yml)

Allow some configurable origin to blacklist some accounts.
Blacklisted accounts are not allowed to perform certain actions.


## Configuration

### Types
* `RuntimeEvent` – The overarching event type.
* `BlacklistingOrigin` – The origin allowed to blacklist accounts. It should be `Root`.
* `WeightInfo` – Information on runtime weights.

## Extrinsics

<details>
<summary><h3>blacklist_account</h3></summary>

Adds an account to the blacklist. Emits the `AccountBlacklisted` event on success.

### Parameters:
  * `origin` – Origin for the call. It should be Root.
  * `accountId` - The account to blacklist.

### Errors:
  * `AccountAlreadyBlacklisted` - The `accountId` was already blacklisted.
</details>

<details>
<summary><h3>remove_blacklisted_account</h3></summary>

Adds an account to the blacklist. Emits the `BlacklistedAccountRemoved` event on success.

### Parameters:
  * `origin` – Origin for the call. It should be Root.
  * `accountId` - The account to remove from the blacklist.

### Errors:
  * `AccountIsNotBlacklisted` - The `accountId` was not previously blacklisted.
</details>

## How to add `pallet-blacklist` to a node

:information_source: The pallet is compatible with Substrate version **[polkadot-v0.9.42](https://github.com/paritytech/substrate/tree/polkadot-v0.9.42).**

:information_source: This section is based on
[Substrate node template](https://github.com/substrate-developer-hub/substrate-node-template/tree/polkadot-v0.9.42).

### Runtime's `Cargo.toml`

Add `pallet-blacklist` to the dependencies:

```toml
[dependencies]
# --snip--
pallet-blacklist = { version = "0.0.1", default-features = false, git = "https://github.com/Moliholy/pallet-blacklist.git" }
# --snip--
```

Update the runtime's `std` feature:
```toml
std = [
    # --snip--
    "pallet-blacklist/std",
    # --snip--
]
```

Configure the blacklist pallet.
```rust
impl pallet_blacklist::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type BlacklistingOrigin = EnsureRoot<AccountId>;
    type WeightInfo = pallet_blacklist::weights::SubstrateWeight<Runtime>;
}
```

Add the pallet to the `construct_runtime` macro call.
```rust
construct_runtime!(
    pub enum Runtime where
        // --snip--
    {
        // --snip---
        Blacklist: pallet_blacklist,
        // --snip---
    }
);
```

Once configured, this pallet **should be used as a valid origin so that any account that is not on the blacklist has a valid origin.**