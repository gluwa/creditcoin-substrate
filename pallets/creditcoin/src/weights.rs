
//! Autogenerated weights for `crate`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-09-21, STEPS: `8`, REPEAT: 8, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/creditcoin-node
// benchmark
// pallet
// -p
// crate
// -e
// *
// --dev
// --execution
// wasm
// --output
// /Users/nathanw/Documents/Work/creditcoin/pallets/creditcoin/src/weights.rs
// --repeat
// 30
// --steps
// 50

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `crate`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> crate::WeightInfo for WeightInfo<T> {
	// Storage: unknown [0xd766358cca00233e6155d7c14e2c085f4e7b9012096b41c4eb3aaf947f6ea429] (r:1 w:1)
	// Storage: Creditcoin PendingTasks (r:1 w:0)
	// Storage: TaskScheduler PendingTasks (r:0 w:1)
	fn migration_v7(t: u32, ) -> Weight {
		Weight::from_ref_time(0 as u64)
			// Standard Error: 237_000
			.saturating_add(Weight::from_ref_time(17_901_000 as u64).saturating_mul(t as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(t as u64)))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(t as u64)))
	}
	// Storage: Creditcoin DealOrders (r:511 w:510)
	// Storage: Creditcoin BidOrders (r:0 w:255)
	// Storage: Creditcoin Offers (r:0 w:255)
	// Storage: Creditcoin PendingTasks (r:0 w:510)
	// Storage: Creditcoin AskOrders (r:0 w:31)
	fn on_initialize(a: u32, b: u32, o: u32, d: u32, f: u32, u: u32, c: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 14_430_000
			.saturating_add((32_968_000 as Weight).saturating_mul(a as Weight))
			// Standard Error: 14_430_000
			.saturating_add((40_160_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 14_430_000
			.saturating_add((42_644_000 as Weight).saturating_mul(o as Weight))
			// Standard Error: 14_430_000
			.saturating_add((53_960_000 as Weight).saturating_mul(d as Weight))
			// Standard Error: 14_430_000
			.saturating_add((50_280_000 as Weight).saturating_mul(f as Weight))
			// Standard Error: 14_430_000
			.saturating_add((36_756_000 as Weight).saturating_mul(u as Weight))
			// Standard Error: 14_430_000
			.saturating_add((25_662_000 as Weight).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(d as Weight)))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(f as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(a as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(b as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(o as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(d as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(f as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(u as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(c as Weight)))
	}
	// Storage: Creditcoin Addresses (r:1 w:1)
	fn register_address() -> Weight {
		(276_602_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Creditcoin LegacyWallets (r:1 w:1)
	// Storage: Creditcoin LegacyBalanceKeeper (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	fn claim_legacy_wallet() -> Weight {
		(71_601_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Creditcoin AskOrders (r:1 w:1)
	// Storage: Creditcoin Addresses (r:1 w:0)
	// Storage: Creditcoin Currencies (r:1 w:0)
	// Storage: Creditcoin UsedGuids (r:1 w:1)
	fn add_ask_order() -> Weight {
		(42_101_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Creditcoin BidOrders (r:1 w:1)
	// Storage: Creditcoin Addresses (r:1 w:0)
	// Storage: Creditcoin Currencies (r:1 w:0)
	// Storage: Creditcoin UsedGuids (r:1 w:1)
	fn add_bid_order() -> Weight {
		(42_001_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Creditcoin AskOrders (r:1 w:0)
	// Storage: Creditcoin BidOrders (r:1 w:0)
	// Storage: Creditcoin Addresses (r:2 w:0)
	// Storage: Creditcoin Offers (r:1 w:1)
	fn add_offer() -> Weight {
		(46_700_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Creditcoin DealOrders (r:1 w:1)
	// Storage: Creditcoin Offers (r:1 w:0)
	// Storage: Creditcoin AskOrders (r:1 w:0)
	// Storage: Creditcoin BidOrders (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	fn add_deal_order() -> Weight {
		(52_700_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Creditcoin Authorities (r:1 w:1)
	fn add_authority() -> Weight {
		(7_100_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Creditcoin Authorities (r:1 w:0)
	// Storage: Creditcoin Transfers (r:1 w:1)
	// Storage: Creditcoin PendingTasks (r:0 w:1)
	fn persist_transfer() -> Weight {
		(41_400_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Creditcoin Authorities (r:1 w:0)
	// Storage: Creditcoin Transfers (r:1 w:0)
	// Storage: Creditcoin PendingTasks (r:0 w:1)
	fn fail_transfer() -> Weight {
		(34_300_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Creditcoin DealOrders (r:1 w:1)
	// Storage: Creditcoin Addresses (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: Creditcoin Transfers (r:1 w:1)
	fn fund_deal_order() -> Weight {
		(58_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Creditcoin DealOrders (r:1 w:1)
	fn lock_deal_order() -> Weight {
		(30_800_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Creditcoin DealOrders (r:1 w:0)
	// Storage: Creditcoin Addresses (r:2 w:0)
	// Storage: Creditcoin Transfers (r:1 w:0)
	// Storage: Creditcoin Currencies (r:1 w:0)
	// Storage: Creditcoin PendingTasks (r:0 w:1)
	fn register_funding_transfer() -> Weight {
		(59_301_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Creditcoin DealOrders (r:1 w:0)
	// Storage: Creditcoin Addresses (r:2 w:0)
	// Storage: Creditcoin Transfers (r:1 w:0)
	// Storage: Creditcoin Currencies (r:1 w:0)
	// Storage: Creditcoin PendingTasks (r:0 w:1)
	fn register_repayment_transfer() -> Weight {
		(52_500_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Creditcoin DealOrders (r:1 w:1)
	// Storage: Creditcoin Addresses (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: Creditcoin Transfers (r:1 w:1)
	fn close_deal_order() -> Weight {
		(58_901_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Creditcoin DealOrders (r:1 w:1)
	// Storage: Creditcoin Addresses (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	fn exempt() -> Weight {
		(42_500_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Creditcoin Currencies (r:1 w:0)
	// Storage: Creditcoin Addresses (r:2 w:0)
	// Storage: Creditcoin AskOrders (r:1 w:1)
	// Storage: Creditcoin BidOrders (r:1 w:1)
	// Storage: Creditcoin Offers (r:1 w:1)
	// Storage: Creditcoin DealOrders (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	fn register_deal_order() -> Weight {
		(337_803_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Creditcoin CollectCoinsContract (r:1 w:0)
	// Storage: Creditcoin CollectedCoins (r:1 w:0)
	// Storage: Creditcoin PendingTasks (r:1 w:1)
	// Storage: Creditcoin Addresses (r:1 w:0)
	fn request_collect_coins() -> Weight {
		(42_600_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Creditcoin Authorities (r:1 w:0)
	// Storage: Creditcoin CollectedCoins (r:1 w:0)
	// Storage: Creditcoin PendingTasks (r:0 w:1)
	fn fail_collect_coins() -> Weight {
		(24_200_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Creditcoin Authorities (r:1 w:0)
	// Storage: Creditcoin CollectedCoins (r:1 w:1)
	// Storage: Creditcoin Addresses (r:1 w:0)
	// Storage: Creditcoin PendingTasks (r:0 w:1)
	fn persist_collect_coins() -> Weight {
		(76_601_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Creditcoin Authorities (r:1 w:1)
	fn remove_authority() -> Weight {
		(8_500_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Creditcoin Currencies (r:1 w:1)
	fn register_currency() -> Weight {
		(7_800_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Creditcoin CollectCoinsContract (r:0 w:1)
	fn set_collect_coins_contract() -> Weight {
		(3_900_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}
