
//! Autogenerated weights for `crate`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-09-20, STEPS: `8`, REPEAT: `8`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `github-runner-6246693080-attempt-1`, CPU: `AMD EPYC 7763 64-Core Processor`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/creditcoin-node
// benchmark
// pallet
// --chain
// dev
// --steps=8
// --repeat=8
// --pallet
// crate
// --extrinsic=*
// --execution
// wasm
// --wasm-execution=compiled
// --heap-pages=10000
// --output
// ./pallets/creditcoin/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `crate`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> crate::WeightInfo for WeightInfo<T> {
	/// Storage: unknown `0xd766358cca00233e6155d7c14e2c085f5e0621c4869aa60c02be9adcc98a0d1d` (r:1025 w:1024)
	/// Proof Skipped: unknown `0xd766358cca00233e6155d7c14e2c085f5e0621c4869aa60c02be9adcc98a0d1d` (r:1025 w:1024)
	/// Storage: TaskScheduler Authorities (r:0 w:1024)
	/// Proof: TaskScheduler Authorities (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// The range of component `t` is `[0, 1024]`.
	fn migration_v7(t: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `153 + t * (53 ±0)`
		//  Estimated: `3600 + t * (2529 ±0)`
		// Minimum execution time: 13_700_000 picoseconds.
		Weight::from_parts(13_801_000, 0)
			.saturating_add(Weight::from_parts(0, 3600))
			// Standard Error: 112_297
			.saturating_add(Weight::from_parts(11_274_786, 0).saturating_mul(t.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(t.into())))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(t.into())))
			.saturating_add(Weight::from_parts(0, 2529).saturating_mul(t.into()))
	}
	/// Storage: Creditcoin AskOrders (r:255 w:255)
	/// Proof: Creditcoin AskOrders (max_values: None, max_size: Some(448), added: 2923, mode: MaxEncodedLen)
	/// Storage: Creditcoin BidOrders (r:255 w:255)
	/// Proof: Creditcoin BidOrders (max_values: None, max_size: Some(448), added: 2923, mode: MaxEncodedLen)
	/// Storage: Creditcoin Offers (r:255 w:255)
	/// Proof: Creditcoin Offers (max_values: None, max_size: Some(415), added: 2890, mode: MaxEncodedLen)
	/// The range of component `a` is `[0, 255]`.
	/// The range of component `b` is `[0, 255]`.
	/// The range of component `o` is `[0, 255]`.
	/// The range of component `d` is `[0, 255]`.
	/// The range of component `f` is `[0, 255]`.
	fn on_initialize(a: u32, b: u32, o: u32, _d: u32, _f: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `201 + a * (69 ±0) + b * (69 ±0) + o * (69 ±0)`
		//  Estimated: `2970 + o * (2890 ±0) + b * (2923 ±0) + a * (2923 ±0)`
		// Minimum execution time: 554_705_000 picoseconds.
		Weight::from_parts(49_494_931, 0)
			.saturating_add(Weight::from_parts(0, 2970))
			// Standard Error: 39_384
			.saturating_add(Weight::from_parts(1_144_961, 0).saturating_mul(a.into()))
			// Standard Error: 39_384
			.saturating_add(Weight::from_parts(1_032_025, 0).saturating_mul(b.into()))
			// Standard Error: 39_384
			.saturating_add(Weight::from_parts(1_054_240, 0).saturating_mul(o.into()))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(a.into())))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(b.into())))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(o.into())))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(a.into())))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(b.into())))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(o.into())))
			.saturating_add(Weight::from_parts(0, 2890).saturating_mul(o.into()))
			.saturating_add(Weight::from_parts(0, 2923).saturating_mul(b.into()))
			.saturating_add(Weight::from_parts(0, 2923).saturating_mul(a.into()))
	}
	/// Storage: Creditcoin Addresses (r:1 w:1)
	/// Proof: Creditcoin Addresses (max_values: None, max_size: Some(597), added: 3072, mode: MaxEncodedLen)
	fn register_address() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `76`
		//  Estimated: `4062`
		// Minimum execution time: 83_101_000 picoseconds.
		Weight::from_parts(91_300_000, 0)
			.saturating_add(Weight::from_parts(0, 4062))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Creditcoin LegacyWallets (r:1 w:1)
	/// Proof: Creditcoin LegacyWallets (max_values: None, max_size: Some(32), added: 2507, mode: MaxEncodedLen)
	/// Storage: Creditcoin LegacyBalanceKeeper (r:1 w:0)
	/// Proof: Creditcoin LegacyBalanceKeeper (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn claim_legacy_wallet() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `288`
		//  Estimated: `8607`
		// Minimum execution time: 81_200_000 picoseconds.
		Weight::from_parts(83_501_000, 0)
			.saturating_add(Weight::from_parts(0, 8607))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Creditcoin AskOrders (r:1 w:1)
	/// Proof: Creditcoin AskOrders (max_values: None, max_size: Some(448), added: 2923, mode: MaxEncodedLen)
	/// Storage: Creditcoin Addresses (r:1 w:0)
	/// Proof: Creditcoin Addresses (max_values: None, max_size: Some(597), added: 3072, mode: MaxEncodedLen)
	/// Storage: Creditcoin UsedGuids (r:1 w:1)
	/// Proof: Creditcoin UsedGuids (max_values: None, max_size: Some(274), added: 2749, mode: MaxEncodedLen)
	fn add_ask_order() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `206`
		//  Estimated: `11714`
		// Minimum execution time: 42_001_000 picoseconds.
		Weight::from_parts(46_401_000, 0)
			.saturating_add(Weight::from_parts(0, 11714))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Creditcoin BidOrders (r:1 w:1)
	/// Proof: Creditcoin BidOrders (max_values: None, max_size: Some(448), added: 2923, mode: MaxEncodedLen)
	/// Storage: Creditcoin Addresses (r:1 w:0)
	/// Proof: Creditcoin Addresses (max_values: None, max_size: Some(597), added: 3072, mode: MaxEncodedLen)
	/// Storage: Creditcoin UsedGuids (r:1 w:1)
	/// Proof: Creditcoin UsedGuids (max_values: None, max_size: Some(274), added: 2749, mode: MaxEncodedLen)
	fn add_bid_order() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `206`
		//  Estimated: `11714`
		// Minimum execution time: 41_000_000 picoseconds.
		Weight::from_parts(44_901_000, 0)
			.saturating_add(Weight::from_parts(0, 11714))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Creditcoin AskOrders (r:1 w:0)
	/// Proof: Creditcoin AskOrders (max_values: None, max_size: Some(448), added: 2923, mode: MaxEncodedLen)
	/// Storage: Creditcoin BidOrders (r:1 w:0)
	/// Proof: Creditcoin BidOrders (max_values: None, max_size: Some(448), added: 2923, mode: MaxEncodedLen)
	/// Storage: Creditcoin Offers (r:1 w:1)
	/// Proof: Creditcoin Offers (max_values: None, max_size: Some(415), added: 2890, mode: MaxEncodedLen)
	fn add_offer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `580`
		//  Estimated: `11706`
		// Minimum execution time: 40_401_000 picoseconds.
		Weight::from_parts(42_100_000, 0)
			.saturating_add(Weight::from_parts(0, 11706))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Creditcoin DealOrders (r:1 w:1)
	/// Proof: Creditcoin DealOrders (max_values: None, max_size: Some(624), added: 3099, mode: MaxEncodedLen)
	/// Storage: Creditcoin Offers (r:1 w:0)
	/// Proof: Creditcoin Offers (max_values: None, max_size: Some(415), added: 2890, mode: MaxEncodedLen)
	/// Storage: Creditcoin AskOrders (r:1 w:0)
	/// Proof: Creditcoin AskOrders (max_values: None, max_size: Some(448), added: 2923, mode: MaxEncodedLen)
	/// Storage: Creditcoin BidOrders (r:1 w:0)
	/// Proof: Creditcoin BidOrders (max_values: None, max_size: Some(448), added: 2923, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	fn add_deal_order() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `854`
		//  Estimated: `17288`
		// Minimum execution time: 52_100_000 picoseconds.
		Weight::from_parts(55_201_000, 0)
			.saturating_add(Weight::from_parts(0, 17288))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: TaskScheduler Authorities (r:1 w:1)
	/// Proof: TaskScheduler Authorities (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	fn add_authority() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `76`
		//  Estimated: `3513`
		// Minimum execution time: 13_401_000 picoseconds.
		Weight::from_parts(14_100_000, 0)
			.saturating_add(Weight::from_parts(0, 3513))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: TaskScheduler Authorities (r:1 w:0)
	/// Proof: TaskScheduler Authorities (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Creditcoin Transfers (r:1 w:1)
	/// Proof: Creditcoin Transfers (max_values: None, max_size: Some(987), added: 3462, mode: MaxEncodedLen)
	/// Storage: TaskScheduler PendingTasks (r:0 w:1)
	/// Proof: TaskScheduler PendingTasks (max_values: None, max_size: Some(1512), added: 3987, mode: MaxEncodedLen)
	fn persist_transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `290`
		//  Estimated: `7965`
		// Minimum execution time: 40_100_000 picoseconds.
		Weight::from_parts(44_000_000, 0)
			.saturating_add(Weight::from_parts(0, 7965))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: TaskScheduler Authorities (r:1 w:0)
	/// Proof: TaskScheduler Authorities (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Creditcoin Transfers (r:1 w:0)
	/// Proof: Creditcoin Transfers (max_values: None, max_size: Some(987), added: 3462, mode: MaxEncodedLen)
	/// Storage: TaskScheduler PendingTasks (r:0 w:1)
	/// Proof: TaskScheduler PendingTasks (max_values: None, max_size: Some(1512), added: 3987, mode: MaxEncodedLen)
	fn fail_transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `290`
		//  Estimated: `7965`
		// Minimum execution time: 30_300_000 picoseconds.
		Weight::from_parts(31_000_000, 0)
			.saturating_add(Weight::from_parts(0, 7965))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Creditcoin DealOrders (r:1 w:1)
	/// Proof: Creditcoin DealOrders (max_values: None, max_size: Some(624), added: 3099, mode: MaxEncodedLen)
	/// Storage: Creditcoin Addresses (r:1 w:0)
	/// Proof: Creditcoin Addresses (max_values: None, max_size: Some(597), added: 3072, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Creditcoin Transfers (r:1 w:1)
	/// Proof: Creditcoin Transfers (max_values: None, max_size: Some(987), added: 3462, mode: MaxEncodedLen)
	fn fund_deal_order() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1039`
		//  Estimated: `14096`
		// Minimum execution time: 56_000_000 picoseconds.
		Weight::from_parts(57_001_000, 0)
			.saturating_add(Weight::from_parts(0, 14096))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Creditcoin DealOrders (r:1 w:1)
	/// Proof: Creditcoin DealOrders (max_values: None, max_size: Some(624), added: 3099, mode: MaxEncodedLen)
	fn lock_deal_order() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `540`
		//  Estimated: `4089`
		// Minimum execution time: 29_900_000 picoseconds.
		Weight::from_parts(30_601_000, 0)
			.saturating_add(Weight::from_parts(0, 4089))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Creditcoin DealOrders (r:1 w:0)
	/// Proof: Creditcoin DealOrders (max_values: None, max_size: Some(624), added: 3099, mode: MaxEncodedLen)
	/// Storage: Creditcoin Addresses (r:2 w:0)
	/// Proof: Creditcoin Addresses (max_values: None, max_size: Some(597), added: 3072, mode: MaxEncodedLen)
	/// Storage: Creditcoin Transfers (r:1 w:0)
	/// Proof: Creditcoin Transfers (max_values: None, max_size: Some(987), added: 3462, mode: MaxEncodedLen)
	/// Storage: TaskScheduler PendingTasks (r:1 w:1)
	/// Proof: TaskScheduler PendingTasks (max_values: None, max_size: Some(1512), added: 3987, mode: MaxEncodedLen)
	fn register_funding_transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `684`
		//  Estimated: `20652`
		// Minimum execution time: 55_100_000 picoseconds.
		Weight::from_parts(58_001_000, 0)
			.saturating_add(Weight::from_parts(0, 20652))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Creditcoin DealOrders (r:1 w:0)
	/// Proof: Creditcoin DealOrders (max_values: None, max_size: Some(624), added: 3099, mode: MaxEncodedLen)
	/// Storage: Creditcoin Addresses (r:2 w:0)
	/// Proof: Creditcoin Addresses (max_values: None, max_size: Some(597), added: 3072, mode: MaxEncodedLen)
	/// Storage: Creditcoin Transfers (r:1 w:0)
	/// Proof: Creditcoin Transfers (max_values: None, max_size: Some(987), added: 3462, mode: MaxEncodedLen)
	/// Storage: TaskScheduler PendingTasks (r:1 w:1)
	/// Proof: TaskScheduler PendingTasks (max_values: None, max_size: Some(1512), added: 3987, mode: MaxEncodedLen)
	fn register_repayment_transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `684`
		//  Estimated: `20652`
		// Minimum execution time: 55_001_000 picoseconds.
		Weight::from_parts(57_201_000, 0)
			.saturating_add(Weight::from_parts(0, 20652))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Creditcoin DealOrders (r:1 w:1)
	/// Proof: Creditcoin DealOrders (max_values: None, max_size: Some(624), added: 3099, mode: MaxEncodedLen)
	/// Storage: Creditcoin Addresses (r:1 w:0)
	/// Proof: Creditcoin Addresses (max_values: None, max_size: Some(597), added: 3072, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Creditcoin Transfers (r:1 w:1)
	/// Proof: Creditcoin Transfers (max_values: None, max_size: Some(987), added: 3462, mode: MaxEncodedLen)
	fn close_deal_order() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1141`
		//  Estimated: `14096`
		// Minimum execution time: 57_100_000 picoseconds.
		Weight::from_parts(58_600_000, 0)
			.saturating_add(Weight::from_parts(0, 14096))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Creditcoin DealOrders (r:1 w:1)
	/// Proof: Creditcoin DealOrders (max_values: None, max_size: Some(624), added: 3099, mode: MaxEncodedLen)
	/// Storage: Creditcoin Addresses (r:1 w:0)
	/// Proof: Creditcoin Addresses (max_values: None, max_size: Some(597), added: 3072, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	fn exempt() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `699`
		//  Estimated: `9644`
		// Minimum execution time: 41_500_000 picoseconds.
		Weight::from_parts(42_100_000, 0)
			.saturating_add(Weight::from_parts(0, 9644))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Creditcoin Addresses (r:2 w:0)
	/// Proof: Creditcoin Addresses (max_values: None, max_size: Some(597), added: 3072, mode: MaxEncodedLen)
	/// Storage: Creditcoin AskOrders (r:1 w:1)
	/// Proof: Creditcoin AskOrders (max_values: None, max_size: Some(448), added: 2923, mode: MaxEncodedLen)
	/// Storage: Creditcoin BidOrders (r:1 w:1)
	/// Proof: Creditcoin BidOrders (max_values: None, max_size: Some(448), added: 2923, mode: MaxEncodedLen)
	/// Storage: Creditcoin Offers (r:1 w:1)
	/// Proof: Creditcoin Offers (max_values: None, max_size: Some(415), added: 2890, mode: MaxEncodedLen)
	/// Storage: Creditcoin DealOrders (r:1 w:1)
	/// Proof: Creditcoin DealOrders (max_values: None, max_size: Some(624), added: 3099, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	fn register_deal_order() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `413`
		//  Estimated: `24422`
		// Minimum execution time: 144_201_000 picoseconds.
		Weight::from_parts(159_101_000, 0)
			.saturating_add(Weight::from_parts(0, 24422))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: TaskScheduler Authorities (r:1 w:1)
	/// Proof: TaskScheduler Authorities (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	fn remove_authority() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `150`
		//  Estimated: `3513`
		// Minimum execution time: 16_300_000 picoseconds.
		Weight::from_parts(18_100_000, 0)
			.saturating_add(Weight::from_parts(0, 3513))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Creditcoin Addresses (r:1 w:1)
	/// Proof: Creditcoin Addresses (max_values: None, max_size: Some(597), added: 3072, mode: MaxEncodedLen)
	fn register_address_v2() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `76`
		//  Estimated: `4062`
		// Minimum execution time: 83_601_000 picoseconds.
		Weight::from_parts(91_900_000, 0)
			.saturating_add(Weight::from_parts(0, 4062))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Creditcoin GATEContract (r:0 w:1)
	/// Proof: Creditcoin GATEContract (max_values: Some(1), max_size: Some(279), added: 774, mode: MaxEncodedLen)
	fn set_gate_contract() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_100_000 picoseconds.
		Weight::from_parts(6_600_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Creditcoin GATEFaucetAccount (r:0 w:1)
	/// Proof: Creditcoin GATEFaucetAccount (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	fn set_gate_faucet() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_000_000 picoseconds.
		Weight::from_parts(6_700_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
