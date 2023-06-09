
//! Autogenerated weights for pallet_blacklist
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-25, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `Joses-MacBook-Pro.local`, CPU: `<UNKNOWN>`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/node-template
// benchmark
// pallet
// --chain
// dev
// --pallet
// pallet_blacklist
// --extrinsic
// *
// --steps=50
// --repeat=20
// --execution=wasm
// --wasm-execution=compiled
// --output
// weights.rs
// --template
// ./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_blacklist.
pub trait WeightInfo {
	fn blacklist_account() -> Weight;
	fn remove_blacklisted_account() -> Weight;
}

/// Weights for pallet_blacklist using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: BlacklistModule BlacklistedAccount (r:1 w:1)
	/// Proof: BlacklistModule BlacklistedAccount (max_values: None, max_size: Some(40), added: 2515, mode: MaxEncodedLen)
	fn blacklist_account() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6`
		//  Estimated: `3505`
		// Minimum execution time: 8_000_000 picoseconds.
		Weight::from_parts(8_000_000, 3505)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: BlacklistModule BlacklistedAccount (r:1 w:1)
	/// Proof: BlacklistModule BlacklistedAccount (max_values: None, max_size: Some(40), added: 2515, mode: MaxEncodedLen)
	fn remove_blacklisted_account() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `70`
		//  Estimated: `3505`
		// Minimum execution time: 8_000_000 picoseconds.
		Weight::from_parts(9_000_000, 3505)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: BlacklistModule BlacklistedAccount (r:1 w:1)
	/// Proof: BlacklistModule BlacklistedAccount (max_values: None, max_size: Some(40), added: 2515, mode: MaxEncodedLen)
	fn blacklist_account() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6`
		//  Estimated: `3505`
		// Minimum execution time: 8_000_000 picoseconds.
		Weight::from_parts(8_000_000, 3505)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: BlacklistModule BlacklistedAccount (r:1 w:1)
	/// Proof: BlacklistModule BlacklistedAccount (max_values: None, max_size: Some(40), added: 2515, mode: MaxEncodedLen)
	fn remove_blacklisted_account() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `70`
		//  Estimated: `3505`
		// Minimum execution time: 8_000_000 picoseconds.
		Weight::from_parts(9_000_000, 3505)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
