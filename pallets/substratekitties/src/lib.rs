#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

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
		// TODO: Implement other attributes for the Kitty struct
		pub owner: T::AccountId,
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

		/// TODO: The maximum amount of kitties a single account can own.
		#[pallet::constant]
		type MaxKittiesOwned: Get<u32>;

		/// TODO: The type of Randomness we want to specify for this pallet.
		type KittyRandomness: Randomness<Self::Hash, BlockNumberFor<Self>>;
	}

	/// TODO: Keeps track of the number of kitties in existence. (hint: using StorageValue)

	/// TODO: Maps the kitty struct to the kitty DNA. (hint: using StorageMap)

	/// TODO: Track the kitties owned by each account. (hint: using StorageMap)

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		// A new kitty was successfully created.
		// TODO: Created { kitty: [u8; 16], owner: T::AccountId },

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
			todo!("create_kitty: create a new kitty with the owner as the extrinsic origin");
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
		// Generates and returns DNA and Gender
		fn gen_dna() -> ([u8; 16], Gender) {
			todo!("gen_dna: Generate a unique DNA for the Kitty");
		}
	}
}
