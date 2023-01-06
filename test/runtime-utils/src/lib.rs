pub mod pool;

use frame_support::{
	sp_runtime::Storage,
	traits::{GenesisBuild, OffchainWorker, OnFinalize, OnInitialize},
};
use frame_system as system;
pub(crate) use parking_lot::RwLock;
use pool::{PoolState, TestTransactionPoolExt};
use sp_arithmetic::traits::{AtLeast32BitUnsigned, One};
pub(crate) use sp_core::offchain::{
	testing::{OffchainState, TestOffchainExt},
	OffchainDbExt, OffchainWorkerExt, TransactionPoolExt,
};
use sp_io::TestExternalities;
use sp_keystore::{testing::KeyStore, KeystoreExt};
use sp_state_machine::BasicExternalities;
pub(crate) use std::sync::Arc;

#[derive(Default)]
pub struct ExtBuilder<G> {
	pub keystore: Option<KeyStore>,
	pool: Option<TestTransactionPoolExt>,
	pub offchain: Option<TestOffchainExt>,
	pub genesis_config: G,
}

impl<G> ExtBuilder<G> {
	pub fn with_keystore(mut self) -> Self {
		self.keystore = Some(KeyStore::new());
		self
	}

	pub fn with_pool(&mut self) -> Arc<RwLock<PoolState>> {
		let (pool, state) = TestTransactionPoolExt::new();
		self.pool = Some(pool);
		state
	}

	pub fn with_offchain(&mut self) -> Arc<RwLock<OffchainState>> {
		let (offchain, state) = TestOffchainExt::new();
		self.offchain = Some(offchain);
		state
	}

	fn system_storage<Config: SystemConfig>() -> Storage {
		system::GenesisConfig::default().build_storage::<Config>().unwrap()
	}

	fn add_capabilities(self, ext: &mut TestExternalities) {
		if let Some(keystore) = self.keystore {
			ext.register_extension(KeystoreExt(Arc::new(keystore)));
		}
		if let Some(pool) = self.pool {
			ext.register_extension(TransactionPoolExt::new(pool));
		}
		if let Some(offchain) = self.offchain {
			ext.register_extension(OffchainDbExt::new(offchain.clone()));
			ext.register_extension(OffchainWorkerExt::new(offchain));
		}
	}

	fn assimilate_pallet_storage<Config, Pallet>(&self, mut storage: Storage) -> Storage
	where
		G: GenesisBuild<Config, Pallet>,
	{
		self.genesis_config.assimilate_storage(&mut storage).unwrap();
		storage
	}

	pub fn build<Config, Pallet>(self) -> TestExternalities
	where
		G: GenesisBuild<Config, Pallet>,
		Config: SystemConfig,
	{
		let storage = Self::system_storage::<Config>();
		let storage = self.assimilate_pallet_storage(storage);
		let mut ext: TestExternalities = storage.into();
		self.add_capabilities(&mut ext);
		ext
	}

	pub fn build_sans_config(self) -> TestExternalities {
		let ext = BasicExternalities::default();
		let mut ext: TestExternalities = ext.into_storages().into();
		self.add_capabilities(&mut ext);
		ext
	}
}

use frame_system::Config as SystemConfig;
use frame_system::Pallet as System;
use std::marker::PhantomData;

pub trait RollTo<BlockNumber> {
	fn with(_: BlockNumber);
}

pub struct Trivial;

impl<BlockNumber> RollTo<BlockNumber> for Trivial {
	fn with(_: BlockNumber) {}
}

pub struct WithWorkerHook<P>(PhantomData<P>);

impl<BlockNumber, Pallet: OffchainWorker<BlockNumber>> RollTo<BlockNumber>
	for WithWorkerHook<Pallet>
{
	fn with(i: BlockNumber) {
		Pallet::offchain_worker(i);
	}
}

pub fn roll_to<T, Runtime, Pallet>(n: Runtime::BlockNumber)
where
	T: RollTo<Runtime::BlockNumber>,
	Runtime: SystemConfig,
	Runtime::BlockNumber: AtLeast32BitUnsigned + One,
	Pallet: OnInitialize<Runtime::BlockNumber> + OnFinalize<Runtime::BlockNumber>,
{
	let mut now = System::<Runtime>::block_number();
	while now < n {
		now += One::one();
		System::<Runtime>::set_block_number(now);
		Pallet::on_initialize(now);
		T::with(now);
		Pallet::on_finalize(now);
	}
}
