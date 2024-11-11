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
  * `account` - The account to blacklist.

### Errors:
  * `AccountAlreadyBlacklisted` - The `accountId` was already blacklisted.
</details>

<details>
<summary><h3>remove_blacklisted_account</h3></summary>

Adds an account to the blacklist. Emits the `BlacklistedAccountRemoved` event on success.

### Parameters:
  * `origin` – Origin for the call. It should be Root.
  * `account` - The account to remove from the blacklist.

### Errors:
  * `AccountIsNotBlacklisted` - The `AccountId` was not previously blacklisted.
</details>

Once configured, this pallet **should be used as a valid origin so that any account that is not on the blacklist has a valid origin.**
