
//! Autogenerated weights for `crate`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-08-12, STEPS: `50`, REPEAT: `30`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `github-runner-10355058378-attempt-2`, CPU: `AMD EPYC 7452 32-Core Processor`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/creditcoin-node
// benchmark
// pallet
// --chain
// dev
// --steps=50
// --repeat=30
// --pallet
// crate
// --extrinsic=*
// --execution
// wasm
// --wasm-execution=compiled
// --heap-pages=10000
// --output
// ./pallets/offchain-task-scheduler/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `crate`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> crate::WeightInfo for WeightInfo<T> {
	/// Storage: TaskScheduler PendingTasks (r:1024 w:1024)
	/// Proof: TaskScheduler PendingTasks (max_values: None, max_size: Some(1512), added: 3987, mode: MaxEncodedLen)
	/// The range of component `t` is `[0, 1024]`.
	fn on_initialize(t: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `136 + t * (69 ±0)`
		//  Estimated: `990 + t * (3987 ±0)`
		// Minimum execution time: 5_900_000 picoseconds.
		Weight::from_parts(6_100_000, 0)
			.saturating_add(Weight::from_parts(0, 990))
			// Standard Error: 819
			.saturating_add(Weight::from_parts(1_303_772, 0).saturating_mul(t.into()))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(t.into())))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(t.into())))
			.saturating_add(Weight::from_parts(0, 3987).saturating_mul(t.into()))
	}
}
