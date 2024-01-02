#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

use codec::{Decode, Encode};
use frame_support::sp_runtime::traits::Hash;
use scale_info::TypeInfo;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{
		pallet_prelude::*,
		traits::{Currency, Randomness},
		Twox64Concat,
	};
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[derive(Encode, Decode, Debug, Clone, PartialEq, TypeInfo)]
	pub enum Gender {
		Male,
		Female,
	}

	#[pallet::config]
	pub trait Config: pallet_balances::Config + frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type WeightInfo: WeightInfo;
		type Currency: Currency<Self::AccountId>;
		type KittyRandomness: Randomness<Self::Hash, BlockNumberFor<Self>>;

		#[pallet::constant]
		type MaxKittyOwned: Get<u32>;
	}

	type AccountOf<T> = <T as frame_system::Config>::AccountId;
	type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	#[pallet::storage]
	#[pallet::getter(fn all_kitties_count)]
	pub(super) type AllKittiesCount<T: Config> = StorageValue<_, u64, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn owned_kitty_count)]
	pub(super) type OwnedKittyCount<T: Config> =
		StorageMap<_, Twox64Concat, T::AccountId, u64, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn kitty_collection)]
	pub type SubstrateKittyCollection<T: Config> =
		StorageMap<_, Twox64Concat, T::Hash, SubstrateKitty<T>>;

	#[pallet::storage]
	#[pallet::getter(fn owner_of)]
	pub(super) type KittyOwner<T: Config> =
		StorageMap<_, Twox64Concat, T::Hash, Option<T::AccountId>, ValueQuery>;

	#[derive(Clone, Encode, Decode, PartialEq, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct SubstrateKitty<T: Config> {
		pub dna: T::Hash,
		pub price: Option<BalanceOf<T>>,
		pub gender: Gender,
		pub owner: AccountOf<T>,
	}

	impl<T: Config> SubstrateKitty<T> {
		fn new(dna: T::Hash, owner: T::AccountId) -> Self {
			SubstrateKitty { dna, gender: Pallet::<T>::generate_gender(dna), owner, price: None }
		}
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		KittyMinted(T::Hash, T::AccountId, BlockNumberFor<T>),
	}

	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
		StorageOverflow,
		DuplicateNewKittyId,
	}

	impl<T: Config> Pallet<T> {
		pub fn generate_gender(random_hash: T::Hash) -> Gender {
			match random_hash.as_ref()[0] % 2 {
				0 => Gender::Male,
				_ => Gender::Female,
			}
		}

		pub fn generate_dna(sender: &T::AccountId) -> T::Hash {
			let (output, block_number) = T::KittyRandomness::random(&b"dna"[..]);
			let payload = (output, block_number, sender);
			T::Hashing::hash_of(&payload)
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(<T as Config>::WeightInfo::mint_new_kitty())]
		pub fn mint_new_kitty(origin: OriginFor<T>) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			let kitty_dna = Pallet::<T>::generate_dna(&sender);
			ensure!(
				!<SubstrateKittyCollection<T>>::contains_key(kitty_dna),
				Error::<T>::DuplicateNewKittyId
			);

			// map the new DNA with the struct data of Kitty
			<SubstrateKittyCollection<T>>::insert(
				kitty_dna,
				SubstrateKitty::<T>::new(kitty_dna, sender.clone()),
			);

			// map the new DNA with its new owner
			ensure!(!<KittyOwner<T>>::contains_key(kitty_dna), Error::<T>::DuplicateNewKittyId);
			<KittyOwner<T>>::insert(kitty_dna, Some(&sender));

			// update the total count of kitties
			let new_all_kitties_count = Self::all_kitties_count()
				.checked_add(1)
				.ok_or(Error::<T>::StorageOverflow)
				.unwrap();

			<AllKittiesCount<T>>::put(new_all_kitties_count);

			let owned_kitty_count = Self::owned_kitty_count(&sender);
			let new_owned_kitty_count =
				owned_kitty_count.checked_add(1).ok_or(Error::<T>::StorageOverflow).unwrap();
			<OwnedKittyCount<T>>::insert(&sender, new_owned_kitty_count);

			Self::deposit_event(Event::KittyMinted(
				kitty_dna,
				sender,
				<frame_system::Pallet<T>>::block_number(),
			));

			Ok(())
		}
	}
}
