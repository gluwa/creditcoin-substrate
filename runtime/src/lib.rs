#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit = "256"]
// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

use frame_election_provider_support::{
	onchain::{self, OnChainExecution},
	SequentialPhragmen,
};
pub use frame_support::traits::EqualPrivilegeOnly;
use frame_support::{
	traits::{ConstU32, ConstU8, U128CurrencyToVote},
	weights::{WeightToFeeCoefficient, WeightToFeeCoefficients, WeightToFeePolynomial},
};
use frame_system::EnsureRoot;
use pallet_babe::AuthorityId as BabeId;
use pallet_creditcoin::weights::WeightInfo as creditcoin_weights;
use pallet_creditcoin::WeightInfo;
use pallet_difficulty::Difficulty as DifficultyT;
use pallet_grandpa::{
	fg_primitives, AuthorityId as GrandpaId, AuthorityList as GrandpaAuthorityList,
};
pub use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use pallet_offchain_task_scheduler::crypto::AuthorityId;
use pallet_session::historical as session_historical;
use pallet_staking::UseValidatorsMap;
pub use pallet_staking_substrate::{self, StakerStatus};
use sp_api::impl_runtime_apis;
use sp_consensus_babe as babe_primitives;
use sp_core::{crypto::KeyTypeId, ConstU64, Encode, OpaqueMetadata};
use sp_runtime::{
	generic, impl_opaque_keys,
	traits::{
		AccountIdLookup, BlakeTwo256, Block as BlockT, Bounded, IdentifyAccount, NumberFor,
		OpaqueKeys, Verify,
	},
	transaction_validity::{TransactionPriority, TransactionSource, TransactionValidity},
	ApplyExtrinsicResult, FixedPointNumber, MultiAddress, MultiSignature, Perquintill,
	SaturatedConversion,
};
use sp_staking::SessionIndex;
use sp_std::prelude::*;
#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;
// A few exports that help ease life for downstream crates.
pub use frame_support::{
	construct_runtime, parameter_types,
	traits::{KeyOwnerProofSystem, Randomness, StorageInfo},
	weights::{
		constants::{
			BlockExecutionWeight, ExtrinsicBaseWeight, ParityDbWeight, WEIGHT_REF_TIME_PER_SECOND,
		},
		IdentityFee, Weight,
	},
	StorageValue,
};

#[cfg(feature = "try-runtime")]
use frame_support::traits::UpgradeCheckSelect;

pub use frame_system::Call as SystemCall;
pub use pallet_balances::Call as BalancesCall;
pub use pallet_timestamp::Call as TimestampCall;
use pallet_transaction_payment::{CurrencyAdapter, Multiplier, TargetedFeeAdjustment};
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;
pub use sp_runtime::{Perbill, Permill};

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

mod version;
pub use version::VERSION;

/// An index to a block.
pub type BlockNumber = u32;

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;

pub type Signer = <Signature as Verify>::Signer;

pub type Moment = u64;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <Signer as IdentifyAccount>::AccountId;

/// Balance of an account.
pub type Balance = u128;

/// Index of a transaction in the chain.
pub type Index = u32;

/// A hash of some data used by the chain.
pub type Hash = sp_core::H256;

pub use opaque::SessionKeys;

/// Opaque types. These are used by the CLI to instantiate machinery that don't need to know
/// the specifics of the runtime. They can then be made to be agnostic over specific formats
/// of data like extrinsics, allowing for them to continue syncing the network through upgrades
/// to even the core data structures.
pub mod opaque {
	use super::*;

	pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;

	/// Opaque block header type.
	pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
	/// Opaque block type.
	pub type Block = generic::Block<Header, UncheckedExtrinsic>;
	/// Opaque block identifier type.
	pub type BlockId = generic::BlockId<Block>;

	pub type SessionHandlers = (Grandpa, Babe);

	impl_opaque_keys! {
		pub struct SessionKeys {
			pub grandpa: Grandpa,
			pub babe: Babe,
			pub im_online: ImOnline,
		}
	}
}

/// This determines the average expected block time that we are targeting.
/// Blocks will be produced at a minimum duration defined by `SLOT_DURATION`.
/// `SLOT_DURATION` is picked up by `pallet_timestamp` which is in turn picked
/// up by `pallet_aura` to implement `fn slot_duration()`.
///
/// Change this to adjust the block time.
pub const MILLISECS_PER_BLOCK: u64 = 6_000;

// NOTE: Currently it is not possible to change the slot duration after the chain has started.
//       Attempting to do so will brick block production.
pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;

// Time is measured by number of blocks.
pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
pub const HOURS: BlockNumber = MINUTES * 60;
pub const DAYS: BlockNumber = HOURS * 24;

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
	NativeVersion { runtime_version: VERSION, can_author_with: Default::default() }
}

const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);

parameter_types! {
	pub const Version: RuntimeVersion = VERSION;
	pub const BlockHashCount: BlockNumber = 2400;
	/// The portion of the `NORMAL_DISPATCH_RATIO` that we adjust the fees with. Blocks filled less
	/// than this will decrease the weight and more will increase.
	pub const TargetBlockFullness: Perquintill = Perquintill::from_percent(25);
	/// The adjustment variable of the runtime. Higher values will cause `TargetBlockFullness` to
	/// change the fees more rapidly.
	pub AdjustmentVariable: Multiplier = Multiplier::saturating_from_rational(3, 100_000);
	/// Minimum amount of the multiplier. This value cannot be too low. A test case should ensure
	/// that combined with `AdjustmentVariable`, we can recover from the minimum.
	/// See `multiplier_can_grow_from_zero`.
	pub MinimumMultiplier: Multiplier = Multiplier::saturating_from_rational(1, 1_000_000u128);
	/// The maximum amount of the multiplier.
	pub MaximumMultiplier: Multiplier = Multiplier::max_value();
	/// We allow for 20 seconds of compute with a 60 second average block time.
	pub BlockWeights: frame_system::limits::BlockWeights = frame_system::limits::BlockWeights
		::with_sensible_defaults(Weight::from_parts(20u64 * WEIGHT_REF_TIME_PER_SECOND, u64::MAX), NORMAL_DISPATCH_RATIO);
	pub BlockLength: frame_system::limits::BlockLength = frame_system::limits::BlockLength
		::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
	pub const SS58Prefix: u8 = 42;

	 pub MaximumSchedulerWeight: Weight = Perbill::from_percent(80) * BlockWeights::get().max_block;
	 pub const MaxScheduledPerBlock: u32 = 50;

}

impl pallet_offchain_task_scheduler::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type UnverifiedTaskTimeout = ConstU32<60>;
	type AuthorityId = AuthorityId;
	type TaskCall = RuntimeCall;
	type WeightInfo = pallet_offchain_task_scheduler::weights::WeightInfo<Runtime>;
	type Task = pallet_creditcoin::Task<AccountId, BlockNumber, Hash, Moment>;
	type Authorship = TaskScheduler;
}

impl pallet_scheduler::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeOrigin = RuntimeOrigin;
	type PalletsOrigin = OriginCaller;
	type RuntimeCall = RuntimeCall;
	type MaximumWeight = MaximumSchedulerWeight;
	type ScheduleOrigin = frame_system::EnsureRoot<AccountId>;
	type MaxScheduledPerBlock = MaxScheduledPerBlock;
	type WeightInfo = ();
	type OriginPrivilegeCmp = EqualPrivilegeOnly;
	type Preimages = ();
}

// Configure FRAME pallets to include in runtime.

impl frame_system::Config for Runtime {
	/// The basic call filter to use in dispatchable.
	type BaseCallFilter = frame_support::traits::Everything;
	/// Block & extrinsics weights: base values and limits.
	type BlockWeights = BlockWeights;
	/// The maximum length of a block (in bytes).
	type BlockLength = BlockLength;
	/// The identifier used to distinguish between accounts.
	type AccountId = AccountId;
	/// The aggregated dispatch type that is available for extrinsics.
	type RuntimeCall = RuntimeCall;
	/// The lookup mechanism to get account ID from whatever is passed in dispatchers.
	type Lookup = AccountIdLookup<AccountId, ()>;
	/// The index type for storing how many extrinsics an account has signed.
	type Index = Index;
	/// The index type for blocks.
	type BlockNumber = BlockNumber;
	/// The type for hashing blocks and tries.
	type Hash = Hash;
	/// The hashing algorithm used.
	type Hashing = BlakeTwo256;
	/// The header type.
	type Header = generic::Header<BlockNumber, BlakeTwo256>;
	/// The ubiquitous event type.
	type RuntimeEvent = RuntimeEvent;
	/// The ubiquitous origin type.
	type RuntimeOrigin = RuntimeOrigin;
	/// Maximum number of block number to block hash mappings to keep (oldest pruned first).
	type BlockHashCount = BlockHashCount;
	/// The weight of database operations that the runtime can invoke.
	type DbWeight = ParityDbWeight;
	/// Version of the runtime.
	type Version = Version;
	/// Converts a module to the index of the module in `construct_runtime!`.
	///
	/// This type is being generated by `construct_runtime!`.
	type PalletInfo = PalletInfo;
	/// What to do if a new account is created.
	type OnNewAccount = ();
	/// What to do if an account is fully reaped from the system.
	type OnKilledAccount = ();
	/// The data to be stored in an account.
	type AccountData = pallet_balances::AccountData<Balance>;
	/// Weight information for the extrinsics of this pallet.
	type SystemWeightInfo = ();
	/// This is used as an identifier of the chain. 42 is the generic substrate prefix.
	type SS58Prefix = SS58Prefix;
	/// The set code logic, just the default since we're not a parachain.
	type OnSetCode = ();

	type MaxConsumers = ConstU32<{ u32::MAX }>;
}

impl pallet_grandpa::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;

	type KeyOwnerProof =
		<Historical as KeyOwnerProofSystem<(KeyTypeId, GrandpaId)>>::Proof;

	type EquivocationReportSystem = ();

	type WeightInfo = ();
	type MaxAuthorities = ConstU32<32>;
	type MaxSetIdSessionEntries = ConstU64<0>;
}

pub const EPOCH_DURATION_IN_BLOCKS: u32 = 1 * MINUTES;

parameter_types! {
	pub const EpochDuration: u64 = EPOCH_DURATION_IN_BLOCKS as u64;
	pub const ExpectedBlockTime: u64 = MILLISECS_PER_BLOCK;

}

// 1 in 4 blocks (on average, not counting collisions) will be primary babe blocks.
// The choice of is done in accordance to the slot duration and expected target
// block time, for safely resisting network delays of maximum two seconds.
// <https://research.web3.foundation/en/latest/polkadot/block-production/Babe.html#-6.-practical-results>
pub const PRIMARY_PROBABILITY: (u64, u64) = (1, 4);

pub const BABE_GENESIS_EPOCH_CONFIG: sp_consensus_babe::BabeEpochConfiguration =
	sp_consensus_babe::BabeEpochConfiguration {
		c: PRIMARY_PROBABILITY,
		allowed_slots: sp_consensus_babe::AllowedSlots::PrimaryAndSecondaryVRFSlots,
	};

impl pallet_babe::Config for Runtime {
	type EpochDuration = EpochDuration;
	type ExpectedBlockTime = ExpectedBlockTime;
	type EpochChangeTrigger = pallet_babe::ExternalTrigger;
	type KeyOwnerProof =
		<Historical as KeyOwnerProofSystem<(KeyTypeId, BabeId)>>::Proof;
	type EquivocationReportSystem = (); // TODO: have an actual equivocation handler
	type WeightInfo = ();
	type MaxAuthorities = ConstU32<128>; // TODO: actually consider what to set this to, I just picked a number
	type DisabledValidators = (); // TODO: Consider whether to use this (through the session pallet)
}

parameter_types! {
	pub const ImOnlineUnsignedPriority: TransactionPriority = TransactionPriority::max_value();
	pub const MaxKeys: u32 = 10_000;
	pub const MaxPeerInHeartbeats: u32 = 10_000;
	pub const MaxPeerDataEncodingSize: u32 = 1_000;
}

impl pallet_im_online::Config for Runtime {
	type AuthorityId = ImOnlineId;
	type RuntimeEvent = RuntimeEvent;
	type ValidatorSet = Historical;
	type NextSessionRotation = Babe;
	type ReportUnresponsiveness = ();
	type UnsignedPriority = ImOnlineUnsignedPriority;
	type WeightInfo = ();
	type MaxKeys = MaxKeys;
	type MaxPeerInHeartbeats = MaxPeerInHeartbeats;
	type MaxPeerDataEncodingSize = MaxPeerDataEncodingSize;
}

impl pallet_authorship::Config for Runtime {
	type FindAuthor = pallet_session::FindAccountFromAuthorIndex<Self, Babe>;
	type EventHandler = (Staking, ImOnline);
}

impl pallet_session::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type ValidatorId = AccountId;
	type ValidatorIdOf = pallet_staking_substrate::StashOf<Self>;
	type ShouldEndSession = Babe;
	type NextSessionRotation = Babe;
	type SessionManager = pallet_session::historical::NoteHistoricalRoot<Self, Staking>;
	type SessionHandler = <opaque::SessionKeys as OpaqueKeys>::KeyTypeIdProviders;
	type Keys = opaque::SessionKeys;
	type WeightInfo = ();
}

parameter_types! {
	// Six sessions in an era (6 hours).
	pub const SessionsPerEra: SessionIndex = 6;

	// 28 eras for unbonding (7 days).
	pub BondingDuration: sp_staking::EraIndex = 28;

	// 27 eras in which slashes can be cancelled (slightly less than 7 days).
	pub SlashDeferDuration: sp_staking::EraIndex = 27;

	/// Setup election pallet to support maximum winners upto 2000. This will mean Staking Pallet
	/// cannot have active validators higher than this count.
	pub const MaxActiveValidators: u32 = 2000;

	pub const MaxNominatorRewardedPerValidator: u32 = 512;
	pub const OffendingValidatorsThreshold: Perbill = Perbill::from_percent(17);
}

pub struct StakingBenchmarkingConfig;
impl pallet_staking_substrate::BenchmarkingConfig for StakingBenchmarkingConfig {
	type MaxValidators = ConstU32<1000>;
	type MaxNominators = ConstU32<1000>;
}

impl pallet_staking_substrate::Config for Runtime {
	type MaxNominations = ConstU32<1000>;
	type Currency = Balances;
	type CurrencyBalance = Balance;
	type UnixTime = Timestamp;
	type CurrencyToVote = U128CurrencyToVote;
	type ElectionProvider = OnChainExecution<OnChainSeqPhragmen>;
	type GenesisElectionProvider = Self::ElectionProvider;
	type RewardRemainder = (); // TODO: do something with this
	type RuntimeEvent = RuntimeEvent;
	type Slash = (); // TODO: do something with this
	type Reward = (); // TODO: do something with this
	type SessionsPerEra = SessionsPerEra;
	type BondingDuration = BondingDuration;
	type SlashDeferDuration = SlashDeferDuration;
	type AdminOrigin = EnsureRoot<Self::AccountId>;
	type SessionInterface = Self;
	type EraPayout = ();
	type NextNewSession = Session;
	type MaxNominatorRewardedPerValidator = MaxNominatorRewardedPerValidator;
	type OffendingValidatorsThreshold = OffendingValidatorsThreshold;
	type VoterList = VoterList;
	type TargetList = UseValidatorsMap<Self>;
	type MaxUnlockingChunks = frame_support::traits::ConstU32<32>;
	type HistoryDepth = frame_support::traits::ConstU32<84>;
	type BenchmarkingConfig = StakingBenchmarkingConfig;
	type OnStakerSlash = ();
	type WeightInfo = ();
}

pub type OnChainAccuracy = sp_runtime::Perbill;

parameter_types! {
	/// We take the top 12500 nominators as electing voters..
	pub const MaxElectingVoters: u32 = 12_500;
	/// ... and all of the validators as electable targets. Whilst this is the case, we cannot and
	/// shall not increase the size of the validator intentions.
	pub const MaxElectableTargets: u16 = u16::MAX;
}

pub struct OnChainSeqPhragmen;
impl onchain::Config for OnChainSeqPhragmen {
	type System = Runtime;
	type Solver = SequentialPhragmen<AccountId, OnChainAccuracy>;
	type DataProvider = Staking;
	type WeightInfo = ();
	type MaxWinners = MaxActiveValidators;
	type VotersBound = MaxElectingVoters;
	type TargetsBound = MaxElectableTargets;
}

type VoterBagsListInstance = pallet_bags_list::Instance1;
impl pallet_bags_list::Config<VoterBagsListInstance> for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type ScoreProvider = Staking;
	type WeightInfo = ();
	type BagThresholds = ();
	type Score = u64;
}

impl session_historical::Config for Runtime {
	type FullIdentification = pallet_staking_substrate::Exposure<AccountId, Balance>;
	type FullIdentificationOf = pallet_staking_substrate::ExposureOf<Runtime>;
}

parameter_types! {
	pub const MinimumPeriod: u64 = SLOT_DURATION / 2;
}

impl pallet_timestamp::Config for Runtime {
	/// A timestamp: milliseconds since the unix epoch.
	type Moment = Moment;
	type OnTimestampSet = Difficulty;
	type MinimumPeriod = MinimumPeriod;
	type WeightInfo = ();
}

parameter_types! {
	pub const ExistentialDeposit: u128 = 500;
	pub const MaxLocks: u32 = 50;
}

impl pallet_balances::Config for Runtime {
	type MaxLocks = MaxLocks;
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	/// The type for recording an account's balance.
	type Balance = Balance;
	/// The ubiquitous event type.
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = pallet_balances::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
	pub const TransactionByteFee: Balance = 1;
}

pub struct LengthToCtcFee;

impl WeightToFeePolynomial for LengthToCtcFee {
	type Balance = Balance;

	fn polynomial() -> WeightToFeeCoefficients<Self::Balance> {
		smallvec::smallvec![WeightToFeeCoefficient {
			coeff_integer: 1,
			coeff_frac: Perbill::zero(),
			negative: false,
			degree: 1,
		}]
	}
}

pub const TARGET_FEE_CREDO: Balance = 10_000_000_000_000_000;

pub const CTC: Balance = 1_000_000_000_000_000_000;

pub struct WeightToCtcFee;

impl WeightToFeePolynomial for WeightToCtcFee {
	type Balance = Balance;

	fn polynomial() -> WeightToFeeCoefficients<Self::Balance> {
		let base = Balance::from(creditcoin_weights::<Runtime>::lock_deal_order().ref_time());
		let ratio = TARGET_FEE_CREDO / base;
		let rem = TARGET_FEE_CREDO % base;
		smallvec::smallvec!(WeightToFeeCoefficient {
			coeff_integer: ratio,
			coeff_frac: Perbill::from_rational(rem, base),
			negative: false,
			degree: 1,
		})
	}
}

/// Parameterized slow adjusting fee updated based on
/// https://research.web3.foundation/en/latest/polkadot/overview/2-token-economics.html#-2.-slow-adjusting-mechanism
pub type SlowAdjustingFeeUpdate<R> = TargetedFeeAdjustment<
	R,
	TargetBlockFullness,
	AdjustmentVariable,
	MinimumMultiplier,
	MaximumMultiplier,
>;

impl pallet_transaction_payment::Config for Runtime {
	type OnChargeTransaction = CurrencyAdapter<Balances, ()>;
	type WeightToFee = WeightToCtcFee;
	type FeeMultiplierUpdate = SlowAdjustingFeeUpdate<Self>;
	type OperationalFeeMultiplier = ConstU8<1u8>;
	type LengthToFee = LengthToCtcFee;
	type RuntimeEvent = RuntimeEvent;
}

impl pallet_sudo::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
}

impl pallet_creditcoin::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Call = RuntimeCall;
	type Signer = Signer;
	type SignerSignature = Signature;
	type HashIntoNonce = Hash;
	type UnverifiedTaskTimeout = ConstU32<60>;
	type WeightInfo = pallet_creditcoin::weights::WeightInfo<Runtime>;
	type TaskScheduler = TaskScheduler;
}

impl pallet_difficulty::Config for Runtime {
	type Moment = Moment;
	type WeightInfo = pallet_difficulty::weights::WeightInfo<Runtime>;
}

impl pallet_rewards::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type WeightInfo = pallet_rewards::weights::WeightInfo<Runtime>;
}

pub type SignedPayload = generic::SignedPayload<RuntimeCall, SignedExtra>;

impl<LocalCall> frame_system::offchain::CreateSignedTransaction<LocalCall> for Runtime
where
	RuntimeCall: From<LocalCall>,
{
	fn create_transaction<C: frame_system::offchain::AppCrypto<Self::Public, Self::Signature>>(
		call: RuntimeCall,
		public: Self::Public,
		account: AccountId,
		nonce: Index,
	) -> Option<(
		RuntimeCall,
		<UncheckedExtrinsic as sp_runtime::traits::Extrinsic>::SignaturePayload,
	)> {
		let period = BlockHashCount::get() as u64;
		let current_block = System::block_number().saturated_into::<u64>().saturating_sub(1);
		let tip = 0;

		let extra: SignedExtra = (
			frame_system::CheckNonZeroSender::<Runtime>::new(),
			frame_system::CheckSpecVersion::<Runtime>::new(),
			frame_system::CheckTxVersion::<Runtime>::new(),
			frame_system::CheckGenesis::<Runtime>::new(),
			frame_system::CheckEra::<Runtime>::from(generic::Era::mortal(period, current_block)),
			frame_system::CheckNonce::<Runtime>::from(nonce),
			frame_system::CheckWeight::<Runtime>::new(),
			pallet_transaction_payment::ChargeTransactionPayment::<Runtime>::from(tip),
		);

		#[cfg_attr(not(feature = "std"), allow(unused_variables))]
		let raw_payload = SignedPayload::new(call, extra)
			.map_err(|e| {
				frame_support::log::warn!("SignedPayload error: {:?}", e);
			})
			.ok()?;

		let signature = raw_payload.using_encoded(|payload| C::sign(payload, public))?;

		let address = MultiAddress::Id(account);
		let (call, extra, _) = raw_payload.deconstruct();
		Some((call, (address, signature, extra)))
	}
}

// Create the runtime by composing the FRAME pallets that were previously configured.
construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = opaque::Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
		System: frame_system,
		Timestamp: pallet_timestamp,
		Babe: pallet_babe,
		Balances: pallet_balances,
		Authorship: pallet_authorship,
		Staking: pallet_staking_substrate,
		Historical: session_historical,
		Session: pallet_session,
		Grandpa: pallet_grandpa,
		ImOnline: pallet_im_online,
		VoterList: pallet_bags_list::<Instance1>,
		TransactionPayment: pallet_transaction_payment,
		Sudo: pallet_sudo,
		Creditcoin: pallet_creditcoin,
		Difficulty: pallet_difficulty,
		Rewards: pallet_rewards,
		Scheduler: pallet_scheduler,
		TaskScheduler: pallet_offchain_task_scheduler,
	}
);

/// The address format for describing accounts.
pub type Address = sp_runtime::MultiAddress<AccountId, ()>;
/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
/// The SignedExtension to the basic transaction logic.
pub type SignedExtra = (
	frame_system::CheckNonZeroSender<Runtime>,
	frame_system::CheckSpecVersion<Runtime>,
	frame_system::CheckTxVersion<Runtime>,
	frame_system::CheckGenesis<Runtime>,
	frame_system::CheckEra<Runtime>,
	frame_system::CheckNonce<Runtime>,
	frame_system::CheckWeight<Runtime>,
	pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
);
/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic =
	generic::UncheckedExtrinsic<Address, RuntimeCall, Signature, SignedExtra>;
/// Executive: handles dispatch to the various modules.
pub type Executive = frame_executive::Executive<
	Runtime,
	Block,
	frame_system::ChainContext<Runtime>,
	Runtime,
	AllPalletsWithSystem,
>;

#[cfg(feature = "runtime-benchmarks")]
#[macro_use]
extern crate frame_benchmarking;

#[cfg(feature = "runtime-benchmarks")]
mod benches {
	define_benchmarks!(
		[frame_benchmarking, BaselineBench::<Runtime>]
		[frame_system, SystemBench::<Runtime>]
		[pallet_balances, Balances]
		[pallet_timestamp, Timestamp]
		[pallet_creditcoin, Creditcoin]
		[pallet_rewards, Rewards]
		[pallet_difficulty, Difficulty]
		[pallet_offchain_task_scheduler, TaskScheduler]
	);
}

impl_runtime_apis! {
	impl sp_api::Core<Block> for Runtime {
		fn version() -> RuntimeVersion {
			VERSION
		}

		fn execute_block(block: Block) {
			Executive::execute_block(block);
		}

		fn initialize_block(header: &<Block as BlockT>::Header) {
			Executive::initialize_block(header)
		}
	}

	impl sp_api::Metadata<Block> for Runtime {
		fn metadata() -> OpaqueMetadata {
			OpaqueMetadata::new(Runtime::metadata().into())
		}
	}

	impl sp_block_builder::BlockBuilder<Block> for Runtime {
		fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
			Executive::apply_extrinsic(extrinsic)
		}

		fn finalize_block() -> <Block as BlockT>::Header {
			Executive::finalize_block()
		}

		fn inherent_extrinsics(data: sp_inherents::InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
			data.create_extrinsics()
		}

		fn check_inherents(
			block: Block,
			data: sp_inherents::InherentData,
		) -> sp_inherents::CheckInherentsResult {
			data.check_extrinsics(&block)
		}
	}

	impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
		fn validate_transaction(
			source: TransactionSource,
			tx: <Block as BlockT>::Extrinsic,
			block_hash: <Block as BlockT>::Hash,
		) -> TransactionValidity {
			Executive::validate_transaction(source, tx, block_hash)
		}
	}

	impl sp_offchain::OffchainWorkerApi<Block> for Runtime {
		fn offchain_worker(header: &<Block as BlockT>::Header) {
			Executive::offchain_worker(header)
		}
	}

	impl sp_session::SessionKeys<Block> for Runtime {
		fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
			opaque::SessionKeys::generate(seed)
		}

		fn decode_session_keys(
			encoded: Vec<u8>,
		) -> Option<Vec<(Vec<u8>, KeyTypeId)>> {
			opaque::SessionKeys::decode_into_raw_public_keys(&encoded)
		}
	}

	impl frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Index> for Runtime {
		fn account_nonce(account: AccountId) -> Index {
			System::account_nonce(account)
		}
	}

	impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance> for Runtime {
		fn query_info(
			uxt: <Block as BlockT>::Extrinsic,
			len: u32,
		) -> pallet_transaction_payment_rpc_runtime_api::RuntimeDispatchInfo<Balance> {
			TransactionPayment::query_info(uxt, len)
		}
		fn query_fee_details(
			uxt: <Block as BlockT>::Extrinsic,
			len: u32,
		) -> pallet_transaction_payment::FeeDetails<Balance> {
			TransactionPayment::query_fee_details(uxt, len)
		}
		fn query_weight_to_fee(weight: Weight) -> Balance {
			TransactionPayment::weight_to_fee(weight)
		}
		fn query_length_to_fee(length: u32) -> Balance {
			TransactionPayment::length_to_fee(length)
		}
	}

	impl sp_consensus_pow::TimestampApi<Block, u64> for Runtime {
		fn timestamp() -> u64 {
			Timestamp::get()
		}
	}

	impl sp_consensus_pow::DifficultyApi<Block, DifficultyT> for Runtime {
		fn difficulty() -> DifficultyT {
			Difficulty::difficulty()
		}
	}

	impl fg_primitives::GrandpaApi<Block> for Runtime {
		fn grandpa_authorities() -> GrandpaAuthorityList {
			Grandpa::grandpa_authorities()
		}

		fn current_set_id() -> fg_primitives::SetId {
			Grandpa::current_set_id()
		}

		fn submit_report_equivocation_unsigned_extrinsic(
			equivocation_proof: fg_primitives::EquivocationProof<
				<Block as BlockT>::Hash,
				NumberFor<Block>,
			>,
			key_owner_proof: fg_primitives::OpaqueKeyOwnershipProof,
		) -> Option<()> {
			let key_owner_proof = key_owner_proof.decode()?;

			Grandpa::submit_unsigned_equivocation_report(
				equivocation_proof,
				key_owner_proof,
			)
		}

		fn generate_key_ownership_proof(
			_set_id: fg_primitives::SetId,
			authority_id: GrandpaId,
		) -> Option<fg_primitives::OpaqueKeyOwnershipProof> {
			use parity_scale_codec::Encode;

			Historical::prove((fg_primitives::KEY_TYPE, authority_id))
				.map(|p| p.encode())
				.map(fg_primitives::OpaqueKeyOwnershipProof::new)
		}
	}

	impl babe_primitives::BabeApi<Block> for Runtime {
		fn configuration() -> babe_primitives::BabeConfiguration {
			let epoch_config = Babe::epoch_config().unwrap_or(BABE_GENESIS_EPOCH_CONFIG);
			babe_primitives::BabeConfiguration {
				slot_duration: Babe::slot_duration(),
				epoch_length: EpochDuration::get(),
				c: epoch_config.c,
				authorities: Babe::authorities().to_vec(),
				randomness: Babe::randomness(),
				allowed_slots: epoch_config.allowed_slots,
			}
		}

		fn current_epoch_start() -> babe_primitives::Slot {
			Babe::current_epoch_start()
		}

		fn current_epoch() -> babe_primitives::Epoch {
			Babe::current_epoch()
		}

		fn next_epoch() -> babe_primitives::Epoch {
			Babe::next_epoch()
		}

		fn generate_key_ownership_proof(
			_slot: babe_primitives::Slot,
			authority_id: babe_primitives::AuthorityId,
		) -> Option<babe_primitives::OpaqueKeyOwnershipProof> {
			Historical::prove((babe_primitives::KEY_TYPE, authority_id))
				.map(|p| p.encode())
				.map(babe_primitives::OpaqueKeyOwnershipProof::new)
		}

		fn submit_report_equivocation_unsigned_extrinsic(
			equivocation_proof: babe_primitives::EquivocationProof<<Block as BlockT>::Header>,
			key_owner_proof: babe_primitives::OpaqueKeyOwnershipProof,
		) -> Option<()> {
			let key_owner_proof = key_owner_proof.decode()?;

			Babe::submit_unsigned_equivocation_report(
				equivocation_proof,
				key_owner_proof,
			)
		}
	}

	#[cfg(feature = "runtime-benchmarks")]
	impl frame_benchmarking::Benchmark<Block> for Runtime {
		fn benchmark_metadata(extra: bool) -> (
			Vec<frame_benchmarking::BenchmarkList>,
			Vec<frame_support::traits::StorageInfo>,
		) {
			use frame_benchmarking::{baseline, Benchmarking, BenchmarkList};
			use frame_support::traits::StorageInfoTrait;
			use frame_system_benchmarking::Pallet as SystemBench;
			use baseline::Pallet as BaselineBench;


			let mut list = Vec::<BenchmarkList>::new();
			list_benchmarks!(list, extra);

			let storage_info = AllPalletsWithSystem::storage_info();

			(list, storage_info)
		}

		fn dispatch_benchmark(
			config: frame_benchmarking::BenchmarkConfig
		) -> Result<Vec<frame_benchmarking::BenchmarkBatch>, sp_runtime::RuntimeString> {
			use frame_benchmarking::{baseline, Benchmarking, BenchmarkBatch, TrackedStorageKey};

			use frame_system_benchmarking::Pallet as SystemBench;
			use baseline::Pallet as BaselineBench;

			impl frame_system_benchmarking::Config for Runtime {}
			impl baseline::Config for Runtime {}

			use frame_support::traits::WhitelistedStorageKeys;
			let whitelist: Vec<TrackedStorageKey> = AllPalletsWithSystem::whitelisted_storage_keys();

			let mut batches = Vec::<BenchmarkBatch>::new();
			let params = (&config, &whitelist);
			add_benchmarks!(params, batches);

			Ok(batches)
		}
	}

	impl task_scheduler_runtime_api::TaskApi<Block, AccountId> for Runtime{
		fn offchain_nonce_key(acc: &AccountId) -> Vec<u8>{
			pallet_offchain_task_scheduler::ocw::nonce_key(acc)
		}
	}

	#[cfg(feature = "try-runtime")]
	impl frame_try_runtime::TryRuntime<Block> for Runtime {
		fn on_runtime_upgrade(checks: UpgradeCheckSelect) -> (Weight, Weight) {
			// NOTE: intentional unwrap: we don't want to propagate the error backwards, and want to
			// have a backtrace here. If any of the pre/post migration checks fail, we shall stop
			// right here and right now.
			let weight = Executive::try_runtime_upgrade(checks).unwrap();
			(weight, BlockWeights::get().max_block)
		}

		fn execute_block(
			block: Block,
			state_root_check: bool,
			signature_check: bool,
			select: frame_try_runtime::TryStateSelect
		) -> Weight {
			// NOTE: intentional unwrap: we don't want to propagate the error backwards, and want to
			// have a backtrace here.
			Executive::try_execute_block(block, state_root_check, signature_check, select).expect("execute-block failed")
		}
	}
}

impl frame_system::offchain::SigningTypes for Runtime {
	type Public = Signer;
	type Signature = Signature;
}

impl<C> frame_system::offchain::SendTransactionTypes<C> for Runtime
where
	RuntimeCall: From<C>,
{
	type OverarchingCall = RuntimeCall;
	type Extrinsic = UncheckedExtrinsic;
}
