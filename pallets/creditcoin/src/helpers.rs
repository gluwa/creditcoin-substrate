mod external_address;

pub use external_address::{address_is_well_formed, generate_external_address};
#[cfg(any(test, feature = "runtime-benchmarks"))]
pub use external_address::{EVMAddress, PublicToAddress};

use crate::{
	pallet::*,
	types::{Address, AddressId},
	DealOrderId, Error, ExternalAmount, ExternalTxId, Guid, Id, OrderId, Task, Transfer,
	TransferId, TransferKind, UnverifiedTransfer,
};
use frame_support::{ensure, traits::Get};
use frame_system::pallet_prelude::*;
use pallet_offchain_task_scheduler::tasks::{TaskScheduler, TaskV2};
use sp_runtime::traits::Saturating;
use sp_std::prelude::*;

#[allow(unused_macros)]
macro_rules! try_get {
	($storage: ident <$t: ident>, $key: expr, $err: ident) => {
		crate::pallet::$storage::<$t>::try_get($key).map_err(|()| crate::pallet::Error::<$t>::$err)
	};
}

#[macro_export]
macro_rules! try_get_id {
	($storage: ident <$t: ident>, $key: expr, $err: ident) => {
		<$crate::pallet::$storage<$t> as DoubleMapExt<_, _, _, _, _, _, _, _, _, _>>::try_get_id(
			$key,
		)
		.map_err(|()| $crate::pallet::Error::<$t>::$err)
	};
}

type DealOrderFor<T> = crate::DealOrder<
	<T as frame_system::Config>::AccountId,
	<T as frame_system::Config>::BlockNumber,
	<T as frame_system::Config>::Hash,
	<T as pallet_timestamp::Config>::Moment,
>;
type TransferFor<T> = crate::Transfer<
	<T as frame_system::Config>::AccountId,
	<T as frame_system::Config>::BlockNumber,
	<T as frame_system::Config>::Hash,
	<T as pallet_timestamp::Config>::Moment,
>;

impl<T: Config> Pallet<T> {
	pub fn block_number() -> BlockNumberFor<T> {
		<frame_system::Pallet<T>>::block_number()
	}
	pub fn timestamp() -> T::Moment {
		<pallet_timestamp::Pallet<T>>::get()
	}
	pub fn get_address(address_id: &AddressId<T::Hash>) -> Result<Address<T::AccountId>, Error<T>> {
		Self::addresses(address_id).ok_or(Error::<T>::NonExistentAddress)
	}

	pub fn try_mutate_deal_order_and_transfer(
		deal_order_id: &DealOrderId<T::BlockNumber, T::Hash>,
		transfer_id: &TransferId<T::Hash>,
		mutate_deal: impl FnOnce(
			&mut DealOrderFor<T>,
		) -> Result<Option<crate::Event<T>>, crate::Error<T>>,
		mutate_transfer: impl FnOnce(
			&mut TransferFor<T>,
			&DealOrderFor<T>,
		) -> Result<Option<crate::Event<T>>, crate::Error<T>>,
	) -> Result<(), crate::Error<T>> {
		let result = DealOrders::<T>::try_mutate(
			deal_order_id.expiration(),
			deal_order_id.hash(),
			|value| {
				let deal_order = value.as_mut().ok_or(crate::Error::<T>::NonExistentDealOrder)?;
				let deal_event = mutate_deal(deal_order)?;

				let transfer_event = Transfers::<T>::try_mutate(transfer_id, |value| {
					let transfer = value.as_mut().ok_or(crate::Error::<T>::NonExistentTransfer)?;
					mutate_transfer(transfer, deal_order)
				})?;

				Ok((deal_event, transfer_event))
			},
		);

		match result {
			Ok((deal_event, transfer_event)) => {
				if let Some(event) = deal_event {
					Self::deposit_event(event);
				}
				if let Some(event) = transfer_event {
					Self::deposit_event(event)
				}

				Ok(())
			},
			Err(e) => Err(e),
		}
	}

	pub fn use_guid(guid: &Guid) -> Result<(), Error<T>> {
		ensure!(!<UsedGuids<T>>::contains_key(guid.clone()), Error::<T>::GuidAlreadyUsed);
		UsedGuids::<T>::insert(guid, ());
		Ok(())
	}

	pub fn register_transfer_internal(
		who: T::AccountId,
		from_id: AddressId<T::Hash>,
		to_id: AddressId<T::Hash>,
		transfer_kind: TransferKind,
		amount: ExternalAmount,
		order_id: OrderId<T::BlockNumber, T::Hash>,
		blockchain_tx_id: ExternalTxId,
	) -> Result<
		(TransferId<T::Hash>, Transfer<T::AccountId, BlockNumberFor<T>, T::Hash, T::Moment>),
		crate::Error<T>,
	> {
		let from = Self::get_address(&from_id)?;
		let to = Self::get_address(&to_id)?;

		ensure!(from.owner == who, Error::<T>::NotAddressOwner);

		ensure!(from.blockchain == to.blockchain, Error::<T>::AddressPlatformMismatch);

		ensure!(from.blockchain.supports(&transfer_kind), Error::<T>::UnsupportedTransferKind);

		let transfer_id = TransferId::new::<T>(&from.blockchain, &blockchain_tx_id);
		ensure!(!Transfers::<T>::contains_key(&transfer_id), Error::<T>::TransferAlreadyRegistered);

		let block = Self::block_number();

		let transfer = Transfer {
			blockchain: from.blockchain,
			kind: transfer_kind,
			amount,
			block,
			from: from_id,
			to: to_id,
			order_id,
			is_processed: false,
			account_id: who,
			tx_id: blockchain_tx_id,
			timestamp: None,
		};

		let deadline = block.saturating_add(T::UnverifiedTaskTimeout::get());

		let pending = UnverifiedTransfer {
			from_external: from.value,
			to_external: to.value,
			transfer: transfer.clone(),
			deadline,
		};
		let task_id = TaskV2::<T>::to_id(&pending);
		T::TaskScheduler::insert(&deadline, &task_id, Task::from(pending));

		Ok((transfer_id, transfer))
	}
}

pub fn non_paying_error<T: Config>(
	error: crate::Error<T>,
) -> frame_support::dispatch::DispatchErrorWithPostInfo {
	frame_support::dispatch::DispatchErrorWithPostInfo {
		error: error.into(),
		post_info: frame_support::dispatch::PostDispatchInfo {
			actual_weight: None,
			pays_fee: frame_support::dispatch::Pays::No,
		},
	}
}

pub mod extensions {

	#[cfg(any(test, feature = "runtime-benchmarks"))]
	#[extend::ext(name = HexToAddress)]
	pub(crate) impl<'a> &'a str {
		fn hex_to_address(self) -> crate::ExternalAddress {
			use sp_std::convert::TryInto;
			hex::decode(self.trim_start_matches("0x")).unwrap().try_into().unwrap()
		}
		fn into_bounded<S>(self) -> frame_support::BoundedVec<u8, S>
		where
			S: frame_support::pallet_prelude::Get<u32>,
		{
			self.as_bytes().into_bounded()
		}
	}

	#[cfg(any(test, feature = "runtime-benchmarks"))]
	#[extend::ext(name = IntoBounded)]
	pub(crate) impl<'a, S, T> &'a [T]
	where
		S: frame_support::pallet_prelude::Get<u32>,
		T: Clone + alloc::fmt::Debug,
	{
		fn try_into_bounded(self) -> Result<frame_support::BoundedVec<T, S>, crate::Vec<T>> {
			core::convert::TryFrom::try_from(self.to_vec())
		}

		fn into_bounded(self) -> frame_support::BoundedVec<T, S> {
			self.try_into_bounded().unwrap()
		}
	}
}
