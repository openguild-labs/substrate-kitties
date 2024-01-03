#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

use frame_support::sp_runtime::traits::Hash;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_support::traits::{Currency, Randomness};
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	// Allows easy access our Pallet's `Balance` type. Comes from `Currency` interface.
	type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	// The Gender type used in the `Kitty` struct
	#[derive(Clone, Encode, Decode, PartialEq, Copy, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub enum Gender {
		Male,
		Female,
	}

	// Struct for holding kitty information
	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen, Copy)]
	#[scale_info(skip_type_params(T))]
	pub struct Kitty<T: Config> {
		// [2-data-structure]: Implement other attributes for the Kitty struct
		pub dna: T::Hash,
		pub price: Option<BalanceOf<T>>,
		pub gender: Gender,
		pub owner: T::AccountId,
	}

	impl<T: Config> Kitty<T> {
		pub fn generate_gender(random_hash: T::Hash) -> Gender {
			match random_hash.as_ref()[0] % 2 {
				0 => Gender::Male,
				_ => Gender::Female,
			}
		}

		fn new(dna: T::Hash, owner: T::AccountId) -> Self {
			Kitty { dna, gender: Kitty::<T>::generate_gender(dna), owner, price: None }
		}
	}

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Type representing the weight of this pallet
		type WeightInfo: WeightInfo;

		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// The Currency handler for the kitties pallet.
		type Currency: Currency<Self::AccountId>;

		/// [2-data-structure]: The maximum amount of kitties a single account can own.
		#[pallet::constant]
		type MaxKittiesOwned: Get<u32>;

		/// [4-onchain-randomness]: The type of Randomness we want to specify for this pallet.
		type KittyRandomness: Randomness<Self::Hash, BlockNumberFor<Self>>;
	}

	/// [2-data-structure]: Keeps track of the number of kitties in existence. (hint: using StorageValue)
	#[pallet::storage]
	#[pallet::getter(fn all_kitties_count)]
	pub(super) type AllKittiesCount<T: Config> = StorageValue<_, u64, ValueQuery>;

	/// [2-data-structure]: Keep track of kitties owned by the owner account
	#[pallet::storage]
	#[pallet::getter(fn kitties_owned)]
	pub(super) type KittiesOwned<T: Config> = StorageMap<
		_,
		Twox64Concat,
		T::AccountId,
		BoundedVec<T::Hash, T::MaxKittiesOwned>,
		ValueQuery,
	>;

	/// [2-data-structure]: Maps the kitty struct to the kitty DNA. (hint: using StorageMap)
	#[pallet::storage]
	#[pallet::getter(fn kitty_collection)]
	pub type Kitties<T: Config> = StorageMap<_, Twox64Concat, T::Hash, Kitty<T>>;

	/// [2-data-structure]: Track the kitties owned by each account. (hint: using StorageMap)
	#[pallet::storage]
	#[pallet::getter(fn owner_of)]
	pub(super) type KittyOwner<T: Config> =
		StorageMap<_, Twox64Concat, T::Hash, Option<T::AccountId>, ValueQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		// A new kitty was successfully created.
		Created { kitty: T::Hash, owner: T::AccountId },
		// A kitty was successfully transferred.
		// TODO: Transferred { from: T::AccountId, to: T::AccountId, kitty: [u8; 16] },

		// The price of a kitty was successfully set.
		// TODO: PriceSet { kitty: [u8; 16], price: Option<BalanceOf<T>> },

		// A kitty was successfully sold.
		// TODO: Sold { seller: T::AccountId, buyer: T::AccountId, kitty: [u8; 16], price: BalanceOf<T> },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// An account may only own `MaxKittiesOwned` kitties.
		TooManyOwned,
		/// This kitty already exists!
		DuplicateKitty,
		/// An overflow has occurred!
		Overflow,
		/// This kitty does not exist!
		NoKitty,
		/// You are not the owner of this kitty.
		NotOwner,
		/// Trying to transfer or buy a kitty from oneself.
		TransferToSelf,
		/// Ensures that the buying price is greater than the asking price.
		BidPriceTooLow,
		/// This kitty is not for sale.
		NotForSale,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create a new unique kitty.
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::create_kitty())]
		pub fn create_kitty(origin: OriginFor<T>) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			let kitty_dna = Pallet::<T>::gen_dna(&sender);
			ensure!(!<Kitties<T>>::contains_key(kitty_dna), Error::<T>::DuplicateKitty);

			// 1. map the new DNA with the struct data of Kitty
			<Kitties<T>>::insert(kitty_dna, Kitty::<T>::new(kitty_dna, sender.clone()));

			// 2. map the new DNA with its new owner
			ensure!(!<KittyOwner<T>>::contains_key(kitty_dna), Error::<T>::DuplicateKitty);
			<KittyOwner<T>>::insert(kitty_dna, Some(&sender));

			// 3. update the total count of kitties
			let new_all_kitties_count =
				Self::all_kitties_count().checked_add(1).ok_or(Error::<T>::Overflow).unwrap();
			<AllKittiesCount<T>>::put(new_all_kitties_count);

			// 4. push the new kitty DNA to the list of existing kitties owned by a sender
			KittiesOwned::<T>::try_append(&sender, kitty_dna)
				.map_err(|_| Error::<T>::TooManyOwned)?;

			// deposit a new event when the kitty is created
			Self::deposit_event(Event::Created { kitty: kitty_dna, owner: sender });

			Ok(())
		}

		/// Directly transfer a kitty to another recipient.
		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::transfer())]
		pub fn transfer(
			origin: OriginFor<T>,
			to: T::AccountId,
			kitty_id: [u8; 16],
		) -> DispatchResult {
			// Any account that holds a kitty can send it to another Account. This will reset the
			// asking price of the kitty, marking it not for sale.
			todo!("transfer: Invoke to transfer Kitty from extrinsic origin to the destination account");
		}

		/// Set the price for a kitty.
		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::set_price())]
		pub fn set_price(
			origin: OriginFor<T>,
			kitty_id: [u8; 16],
			new_price: Option<BalanceOf<T>>,
		) -> DispatchResult {
			todo!(
				"set_price: listing the kitty on the marketplace by setting price so other can buy"
			);
		}

		/// Buy a saleable kitty. The bid price provided from the buyer has to be equal or higher
		/// than the ask price from the seller.
		#[pallet::call_index(5)]
		#[pallet::weight(T::WeightInfo::buy_kitty())]
		pub fn buy_kitty(
			origin: OriginFor<T>,
			kitty_id: [u8; 16],
			bid_price: BalanceOf<T>,
		) -> DispatchResult {
			todo!("buy_kitty: Implement a method to buy a kitty from the marketplace.");
		}
	}

	// Pallet's internal functions.
	impl<T: Config> Pallet<T> {
		// [4-onchain-randomness] Generates and returns DNA and Gender
		fn gen_dna(minter: &T::AccountId) -> T::Hash {
			let (output, block_number) = T::KittyRandomness::random(&b"dna"[..]);
			let payload = (output, block_number, minter);
			T::Hashing::hash_of(&payload)
		}
	}
}
