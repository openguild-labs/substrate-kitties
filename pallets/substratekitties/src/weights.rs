#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_template.
pub trait WeightInfo {
	fn create_kitty() -> Weight;
	fn transfer() -> Weight;
	fn set_price() -> Weight;
	fn buy_kitty() -> Weight;
}

/// Weights for pallet_template using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn create_kitty() -> Weight {
		Weight::from_parts(9_000_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}

	fn transfer() -> Weight {
		Weight::from_parts(9_000_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}

	fn set_price() -> Weight {
		Weight::from_parts(9_000_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}

	fn buy_kitty() -> Weight {
		Weight::from_parts(9_000_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn create_kitty() -> Weight {
		Weight::from_parts(9_000_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}

	fn transfer() -> Weight {
		Weight::from_parts(9_000_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}

	fn set_price() -> Weight {
		Weight::from_parts(9_000_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}

	fn buy_kitty() -> Weight {
		Weight::from_parts(9_000_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
