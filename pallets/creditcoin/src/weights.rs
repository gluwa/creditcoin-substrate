
//! Autogenerated weights for `crate`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-04-13, STEPS: `8`, REPEAT: `8`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `github-runner-4687283919-attempt-1`, CPU: `AMD EPYC 7452 32-Core Processor`
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
	/// Storage: unknown `0xd766358cca00233e6155d7c14e2c085f958b219926181a115ab05df09ebaabf5` (r:1 w:0)
	/// Proof Skipped: unknown `0xd766358cca00233e6155d7c14e2c085f958b219926181a115ab05df09ebaabf5` (r:1 w:0)
	/// The range of component `t` is `[0, 1024]`.
	fn migration_v7(t: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `84`
		//  Estimated: `3533`
		// Minimum execution time: 5_900_000 picoseconds.
		Weight::from_parts(7_507_747, 0)
			.saturating_add(Weight::from_parts(0, 3533))
			// Standard Error: 421
			.saturating_add(Weight::from_parts(3_082, 0).saturating_mul(t.into()))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	/// Storage: unknown `0xd766358cca00233e6155d7c14e2c085f5e0621c4869aa60c02be9adcc98a0d1d` (r:1025 w:1024)
	/// Proof Skipped: unknown `0xd766358cca00233e6155d7c14e2c085f5e0621c4869aa60c02be9adcc98a0d1d` (r:1025 w:1024)
	/// Storage: TaskScheduler Authorities (r:0 w:1024)
	/// Proof: TaskScheduler Authorities (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// The range of component `t` is `[0, 1024]`.
	fn migration_v8(t: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `120 + t * (53 ±0)`
		//  Estimated: `3567 + t * (2529 ±0)`
		// Minimum execution time: 14_100_000 picoseconds.
		Weight::from_parts(14_301_000, 0)
			.saturating_add(Weight::from_parts(0, 3567))
			// Standard Error: 102_150
			.saturating_add(Weight::from_parts(13_045_847, 0).saturating_mul(t.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(t.into())))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(t.into())))
			.saturating_add(Weight::from_parts(0, 2529).saturating_mul(t.into()))
	}
	/// Storage: Creditcoin AskOrders (r:255 w:255)
	/// Proof: Creditcoin AskOrders (max_values: None, max_size: Some(221), added: 2696, mode: MaxEncodedLen)
	/// Storage: Creditcoin BidOrders (r:255 w:255)
	/// Proof: Creditcoin BidOrders (max_values: None, max_size: Some(221), added: 2696, mode: MaxEncodedLen)
	/// Storage: Creditcoin Offers (r:255 w:255)
	/// Proof: Creditcoin Offers (max_values: None, max_size: Some(156), added: 2631, mode: MaxEncodedLen)
	/// Storage: Creditcoin DealOrders (r:510 w:509)
	/// Proof: Creditcoin DealOrders (max_values: None, max_size: Some(397), added: 2872, mode: MaxEncodedLen)
	/// The range of component `a` is `[0, 255]`.
	/// The range of component `b` is `[0, 255]`.
	/// The range of component `o` is `[0, 255]`.
	/// The range of component `d` is `[0, 255]`.
	/// The range of component `f` is `[0, 255]`.
	fn on_initialize(a: u32, b: u32, o: u32, d: u32, f: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `172 + a * (69 ±0) + b * (69 ±0) + o * (69 ±0) + d * (292 ±0) + f * (324 ±0)`
		//  Estimated: `6832 + d * (2872 ±0) + f * (2872 ±0) + b * (2696 ±0) + o * (2631 ±0) + a * (2696 ±0)`
		// Minimum execution time: 2_935_702_000 picoseconds.
		Weight::from_parts(2_988_904_000, 0)
			.saturating_add(Weight::from_parts(0, 6832))
			// Standard Error: 241_020
			.saturating_add(Weight::from_parts(5_377_270, 0).saturating_mul(d.into()))
			// Standard Error: 241_020
			.saturating_add(Weight::from_parts(10_581_157, 0).saturating_mul(f.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(a.into())))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(b.into())))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(o.into())))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(d.into())))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(f.into())))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(a.into())))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(b.into())))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(o.into())))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(d.into())))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(f.into())))
			.saturating_add(Weight::from_parts(0, 2872).saturating_mul(d.into()))
			.saturating_add(Weight::from_parts(0, 2872).saturating_mul(f.into()))
			.saturating_add(Weight::from_parts(0, 2696).saturating_mul(b.into()))
			.saturating_add(Weight::from_parts(0, 2631).saturating_mul(o.into()))
			.saturating_add(Weight::from_parts(0, 2696).saturating_mul(a.into()))
	}
	/// Storage: Creditcoin Addresses (r:1 w:1)
	/// Proof: Creditcoin Addresses (max_values: None, max_size: Some(347), added: 2822, mode: MaxEncodedLen)
	fn register_address() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `3812`
		// Minimum execution time: 92_203_000 picoseconds.
		Weight::from_parts(95_404_000, 0)
			.saturating_add(Weight::from_parts(0, 3812))
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
		//  Measured:  `254`
		//  Estimated: `8607`
		// Minimum execution time: 89_803_000 picoseconds.
		Weight::from_parts(91_103_000, 0)
			.saturating_add(Weight::from_parts(0, 8607))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Creditcoin AskOrders (r:1 w:1)
	/// Proof: Creditcoin AskOrders (max_values: None, max_size: Some(221), added: 2696, mode: MaxEncodedLen)
	/// Storage: Creditcoin Addresses (r:1 w:0)
	/// Proof: Creditcoin Addresses (max_values: None, max_size: Some(347), added: 2822, mode: MaxEncodedLen)
	/// Storage: Creditcoin Currencies (r:1 w:0)
	/// Proof: Creditcoin Currencies (max_values: None, max_size: Some(303), added: 2778, mode: MaxEncodedLen)
	/// Storage: Creditcoin UsedGuids (r:1 w:1)
	/// Proof: Creditcoin UsedGuids (max_values: None, max_size: Some(274), added: 2749, mode: MaxEncodedLen)
	fn add_ask_order() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `253`
		//  Estimated: `15005`
		// Minimum execution time: 52_202_000 picoseconds.
		Weight::from_parts(57_402_000, 0)
			.saturating_add(Weight::from_parts(0, 15005))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Creditcoin BidOrders (r:1 w:1)
	/// Proof: Creditcoin BidOrders (max_values: None, max_size: Some(221), added: 2696, mode: MaxEncodedLen)
	/// Storage: Creditcoin Addresses (r:1 w:0)
	/// Proof: Creditcoin Addresses (max_values: None, max_size: Some(347), added: 2822, mode: MaxEncodedLen)
	/// Storage: Creditcoin Currencies (r:1 w:0)
	/// Proof: Creditcoin Currencies (max_values: None, max_size: Some(303), added: 2778, mode: MaxEncodedLen)
	/// Storage: Creditcoin UsedGuids (r:1 w:1)
	/// Proof: Creditcoin UsedGuids (max_values: None, max_size: Some(274), added: 2749, mode: MaxEncodedLen)
	fn add_bid_order() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `253`
		//  Estimated: `15005`
		// Minimum execution time: 52_602_000 picoseconds.
		Weight::from_parts(53_902_000, 0)
			.saturating_add(Weight::from_parts(0, 15005))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Creditcoin AskOrders (r:1 w:0)
	/// Proof: Creditcoin AskOrders (max_values: None, max_size: Some(221), added: 2696, mode: MaxEncodedLen)
	/// Storage: Creditcoin BidOrders (r:1 w:0)
	/// Proof: Creditcoin BidOrders (max_values: None, max_size: Some(221), added: 2696, mode: MaxEncodedLen)
	/// Storage: Creditcoin Addresses (r:2 w:0)
	/// Proof: Creditcoin Addresses (max_values: None, max_size: Some(347), added: 2822, mode: MaxEncodedLen)
	/// Storage: Creditcoin Offers (r:1 w:1)
	/// Proof: Creditcoin Offers (max_values: None, max_size: Some(156), added: 2631, mode: MaxEncodedLen)
	fn add_offer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `849`
		//  Estimated: `17627`
		// Minimum execution time: 60_302_000 picoseconds.
		Weight::from_parts(67_503_000, 0)
			.saturating_add(Weight::from_parts(0, 17627))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Creditcoin DealOrders (r:1 w:1)
	/// Proof: Creditcoin DealOrders (max_values: None, max_size: Some(397), added: 2872, mode: MaxEncodedLen)
	/// Storage: Creditcoin Offers (r:1 w:0)
	/// Proof: Creditcoin Offers (max_values: None, max_size: Some(156), added: 2631, mode: MaxEncodedLen)
	/// Storage: Creditcoin AskOrders (r:1 w:0)
	/// Proof: Creditcoin AskOrders (max_values: None, max_size: Some(221), added: 2696, mode: MaxEncodedLen)
	/// Storage: Creditcoin BidOrders (r:1 w:0)
	/// Proof: Creditcoin BidOrders (max_values: None, max_size: Some(221), added: 2696, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	fn add_deal_order() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `877`
		//  Estimated: `16348`
		// Minimum execution time: 61_302_000 picoseconds.
		Weight::from_parts(61_902_000, 0)
			.saturating_add(Weight::from_parts(0, 16348))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: TaskScheduler Authorities (r:1 w:1)
	/// Proof: TaskScheduler Authorities (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	fn add_authority() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `3513`
		// Minimum execution time: 12_901_000 picoseconds.
		Weight::from_parts(14_201_000, 0)
			.saturating_add(Weight::from_parts(0, 3513))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: TaskScheduler Authorities (r:1 w:0)
	/// Proof: TaskScheduler Authorities (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Creditcoin Transfers (r:1 w:1)
	/// Proof: Creditcoin Transfers (max_values: None, max_size: Some(479), added: 2954, mode: MaxEncodedLen)
	/// Storage: TaskScheduler PendingTasks (r:0 w:1)
	/// Proof: TaskScheduler PendingTasks (max_values: None, max_size: Some(1276), added: 3751, mode: MaxEncodedLen)
	fn persist_transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `290`
		//  Estimated: `7457`
		// Minimum execution time: 44_001_000 picoseconds.
		Weight::from_parts(44_902_000, 0)
			.saturating_add(Weight::from_parts(0, 7457))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: TaskScheduler Authorities (r:1 w:0)
	/// Proof: TaskScheduler Authorities (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Creditcoin Transfers (r:1 w:0)
	/// Proof: Creditcoin Transfers (max_values: None, max_size: Some(479), added: 2954, mode: MaxEncodedLen)
	/// Storage: TaskScheduler PendingTasks (r:0 w:1)
	/// Proof: TaskScheduler PendingTasks (max_values: None, max_size: Some(1276), added: 3751, mode: MaxEncodedLen)
	fn fail_transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `290`
		//  Estimated: `7457`
		// Minimum execution time: 32_101_000 picoseconds.
		Weight::from_parts(32_601_000, 0)
			.saturating_add(Weight::from_parts(0, 7457))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Creditcoin DealOrders (r:1 w:1)
	/// Proof: Creditcoin DealOrders (max_values: None, max_size: Some(397), added: 2872, mode: MaxEncodedLen)
	/// Storage: Creditcoin Addresses (r:1 w:0)
	/// Proof: Creditcoin Addresses (max_values: None, max_size: Some(347), added: 2822, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Creditcoin Transfers (r:1 w:1)
	/// Proof: Creditcoin Transfers (max_values: None, max_size: Some(479), added: 2954, mode: MaxEncodedLen)
	fn fund_deal_order() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `991`
		//  Estimated: `13111`
		// Minimum execution time: 64_802_000 picoseconds.
		Weight::from_parts(65_302_000, 0)
			.saturating_add(Weight::from_parts(0, 13111))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Creditcoin DealOrders (r:1 w:1)
	/// Proof: Creditcoin DealOrders (max_values: None, max_size: Some(397), added: 2872, mode: MaxEncodedLen)
	fn lock_deal_order() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `570`
		//  Estimated: `3862`
		// Minimum execution time: 30_601_000 picoseconds.
		Weight::from_parts(31_301_000, 0)
			.saturating_add(Weight::from_parts(0, 3862))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Creditcoin DealOrders (r:1 w:0)
	/// Proof: Creditcoin DealOrders (max_values: None, max_size: Some(397), added: 2872, mode: MaxEncodedLen)
	/// Storage: Creditcoin Addresses (r:2 w:0)
	/// Proof: Creditcoin Addresses (max_values: None, max_size: Some(347), added: 2822, mode: MaxEncodedLen)
	/// Storage: Creditcoin Currencies (r:1 w:0)
	/// Proof: Creditcoin Currencies (max_values: None, max_size: Some(303), added: 2778, mode: MaxEncodedLen)
	/// Storage: Creditcoin Transfers (r:1 w:0)
	/// Proof: Creditcoin Transfers (max_values: None, max_size: Some(479), added: 2954, mode: MaxEncodedLen)
	/// Storage: TaskScheduler PendingTasks (r:1 w:1)
	/// Proof: TaskScheduler PendingTasks (max_values: None, max_size: Some(1276), added: 3751, mode: MaxEncodedLen)
	fn register_funding_transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `764`
		//  Estimated: `22949`
		// Minimum execution time: 69_202_000 picoseconds.
		Weight::from_parts(70_203_000, 0)
			.saturating_add(Weight::from_parts(0, 22949))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Creditcoin DealOrders (r:1 w:0)
	/// Proof: Creditcoin DealOrders (max_values: None, max_size: Some(397), added: 2872, mode: MaxEncodedLen)
	/// Storage: Creditcoin Addresses (r:2 w:0)
	/// Proof: Creditcoin Addresses (max_values: None, max_size: Some(347), added: 2822, mode: MaxEncodedLen)
	/// Storage: Creditcoin Currencies (r:1 w:0)
	/// Proof: Creditcoin Currencies (max_values: None, max_size: Some(303), added: 2778, mode: MaxEncodedLen)
	/// Storage: Creditcoin Transfers (r:1 w:0)
	/// Proof: Creditcoin Transfers (max_values: None, max_size: Some(479), added: 2954, mode: MaxEncodedLen)
	/// Storage: TaskScheduler PendingTasks (r:1 w:1)
	/// Proof: TaskScheduler PendingTasks (max_values: None, max_size: Some(1276), added: 3751, mode: MaxEncodedLen)
	fn register_repayment_transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `764`
		//  Estimated: `22949`
		// Minimum execution time: 69_102_000 picoseconds.
		Weight::from_parts(70_102_000, 0)
			.saturating_add(Weight::from_parts(0, 22949))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Creditcoin DealOrders (r:1 w:1)
	/// Proof: Creditcoin DealOrders (max_values: None, max_size: Some(397), added: 2872, mode: MaxEncodedLen)
	/// Storage: Creditcoin Addresses (r:2 w:0)
	/// Proof: Creditcoin Addresses (max_values: None, max_size: Some(347), added: 2822, mode: MaxEncodedLen)
	/// Storage: Creditcoin Transfers (r:1 w:0)
	/// Proof: Creditcoin Transfers (max_values: None, max_size: Some(479), added: 2954, mode: MaxEncodedLen)
	/// Storage: TaskScheduler PendingTasks (r:1 w:1)
	/// Proof: TaskScheduler PendingTasks (max_values: None, max_size: Some(1276), added: 3751, mode: MaxEncodedLen)
	fn register_funding_transfer_legacy() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `709`
		//  Estimated: `19181`
		// Minimum execution time: 72_302_000 picoseconds.
		Weight::from_parts(74_202_000, 0)
			.saturating_add(Weight::from_parts(0, 19181))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Creditcoin DealOrders (r:1 w:1)
	/// Proof: Creditcoin DealOrders (max_values: None, max_size: Some(397), added: 2872, mode: MaxEncodedLen)
	/// Storage: Creditcoin Addresses (r:2 w:0)
	/// Proof: Creditcoin Addresses (max_values: None, max_size: Some(347), added: 2822, mode: MaxEncodedLen)
	/// Storage: Creditcoin Transfers (r:1 w:0)
	/// Proof: Creditcoin Transfers (max_values: None, max_size: Some(479), added: 2954, mode: MaxEncodedLen)
	/// Storage: TaskScheduler PendingTasks (r:1 w:1)
	/// Proof: TaskScheduler PendingTasks (max_values: None, max_size: Some(1276), added: 3751, mode: MaxEncodedLen)
	fn register_repayment_transfer_legacy() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `709`
		//  Estimated: `19181`
		// Minimum execution time: 72_202_000 picoseconds.
		Weight::from_parts(72_702_000, 0)
			.saturating_add(Weight::from_parts(0, 19181))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Creditcoin DealOrders (r:1 w:1)
	/// Proof: Creditcoin DealOrders (max_values: None, max_size: Some(397), added: 2872, mode: MaxEncodedLen)
	/// Storage: Creditcoin Addresses (r:1 w:0)
	/// Proof: Creditcoin Addresses (max_values: None, max_size: Some(347), added: 2822, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Creditcoin Transfers (r:1 w:1)
	/// Proof: Creditcoin Transfers (max_values: None, max_size: Some(479), added: 2954, mode: MaxEncodedLen)
	fn close_deal_order() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1093`
		//  Estimated: `13111`
		// Minimum execution time: 65_502_000 picoseconds.
		Weight::from_parts(66_302_000, 0)
			.saturating_add(Weight::from_parts(0, 13111))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Creditcoin DealOrders (r:1 w:1)
	/// Proof: Creditcoin DealOrders (max_values: None, max_size: Some(397), added: 2872, mode: MaxEncodedLen)
	/// Storage: Creditcoin Addresses (r:1 w:0)
	/// Proof: Creditcoin Addresses (max_values: None, max_size: Some(347), added: 2822, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	fn exempt() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `693`
		//  Estimated: `9167`
		// Minimum execution time: 46_101_000 picoseconds.
		Weight::from_parts(47_102_000, 0)
			.saturating_add(Weight::from_parts(0, 9167))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Creditcoin Currencies (r:1 w:0)
	/// Proof: Creditcoin Currencies (max_values: None, max_size: Some(303), added: 2778, mode: MaxEncodedLen)
	/// Storage: Creditcoin Addresses (r:2 w:0)
	/// Proof: Creditcoin Addresses (max_values: None, max_size: Some(347), added: 2822, mode: MaxEncodedLen)
	/// Storage: Creditcoin AskOrders (r:1 w:1)
	/// Proof: Creditcoin AskOrders (max_values: None, max_size: Some(221), added: 2696, mode: MaxEncodedLen)
	/// Storage: Creditcoin BidOrders (r:1 w:1)
	/// Proof: Creditcoin BidOrders (max_values: None, max_size: Some(221), added: 2696, mode: MaxEncodedLen)
	/// Storage: Creditcoin Offers (r:1 w:1)
	/// Proof: Creditcoin Offers (max_values: None, max_size: Some(156), added: 2631, mode: MaxEncodedLen)
	/// Storage: Creditcoin DealOrders (r:1 w:1)
	/// Proof: Creditcoin DealOrders (max_values: None, max_size: Some(397), added: 2872, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	fn register_deal_order() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `424`
		//  Estimated: `26750`
		// Minimum execution time: 172_006_000 picoseconds.
		Weight::from_parts(190_307_000, 0)
			.saturating_add(Weight::from_parts(0, 26750))
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Creditcoin CollectCoinsContract (r:1 w:0)
	/// Proof: Creditcoin CollectCoinsContract (max_values: Some(1), max_size: Some(29), added: 524, mode: MaxEncodedLen)
	/// Storage: Creditcoin CollectedCoins (r:1 w:0)
	/// Proof: Creditcoin CollectedCoins (max_values: None, max_size: Some(338), added: 2813, mode: MaxEncodedLen)
	/// Storage: TaskScheduler PendingTasks (r:1 w:1)
	/// Proof: TaskScheduler PendingTasks (max_values: None, max_size: Some(1276), added: 3751, mode: MaxEncodedLen)
	/// Storage: Creditcoin Addresses (r:1 w:0)
	/// Proof: Creditcoin Addresses (max_values: None, max_size: Some(347), added: 2822, mode: MaxEncodedLen)
	fn request_collect_coins() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `177`
		//  Estimated: `13870`
		// Minimum execution time: 47_601_000 picoseconds.
		Weight::from_parts(51_302_000, 0)
			.saturating_add(Weight::from_parts(0, 13870))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: TaskScheduler Authorities (r:1 w:0)
	/// Proof: TaskScheduler Authorities (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Creditcoin CollectedCoins (r:1 w:0)
	/// Proof: Creditcoin CollectedCoins (max_values: None, max_size: Some(338), added: 2813, mode: MaxEncodedLen)
	/// Storage: TaskScheduler PendingTasks (r:0 w:1)
	/// Proof: TaskScheduler PendingTasks (max_values: None, max_size: Some(1276), added: 3751, mode: MaxEncodedLen)
	fn fail_collect_coins() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `120`
		//  Estimated: `7316`
		// Minimum execution time: 28_301_000 picoseconds.
		Weight::from_parts(28_901_000, 0)
			.saturating_add(Weight::from_parts(0, 7316))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: TaskScheduler Authorities (r:1 w:0)
	/// Proof: TaskScheduler Authorities (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Creditcoin CollectedCoins (r:1 w:1)
	/// Proof: Creditcoin CollectedCoins (max_values: None, max_size: Some(338), added: 2813, mode: MaxEncodedLen)
	/// Storage: Creditcoin Addresses (r:1 w:0)
	/// Proof: Creditcoin Addresses (max_values: None, max_size: Some(347), added: 2822, mode: MaxEncodedLen)
	/// Storage: TaskScheduler PendingTasks (r:0 w:1)
	/// Proof: TaskScheduler PendingTasks (max_values: None, max_size: Some(1276), added: 3751, mode: MaxEncodedLen)
	fn persist_collect_coins() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `251`
		//  Estimated: `11128`
		// Minimum execution time: 88_703_000 picoseconds.
		Weight::from_parts(91_003_000, 0)
			.saturating_add(Weight::from_parts(0, 11128))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: TaskScheduler Authorities (r:1 w:1)
	/// Proof: TaskScheduler Authorities (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	fn remove_authority() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `116`
		//  Estimated: `3513`
		// Minimum execution time: 17_601_000 picoseconds.
		Weight::from_parts(17_900_000, 0)
			.saturating_add(Weight::from_parts(0, 3513))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Creditcoin Currencies (r:1 w:1)
	/// Proof: Creditcoin Currencies (max_values: None, max_size: Some(303), added: 2778, mode: MaxEncodedLen)
	fn register_currency() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `3768`
		// Minimum execution time: 24_601_000 picoseconds.
		Weight::from_parts(25_301_000, 0)
			.saturating_add(Weight::from_parts(0, 3768))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Creditcoin CollectCoinsContract (r:0 w:1)
	/// Proof: Creditcoin CollectCoinsContract (max_values: Some(1), max_size: Some(29), added: 524, mode: MaxEncodedLen)
	fn set_collect_coins_contract() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 10_100_000 picoseconds.
		Weight::from_parts(15_301_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
