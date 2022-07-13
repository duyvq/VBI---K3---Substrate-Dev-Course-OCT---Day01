#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

// #[cfg(test)]
// mod mock;

// #[cfg(test)]
// mod tests;

// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;

use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
use frame_support::inherent::Vec;
use frame_support::dispatch::{fmt, DispatchError};

#[frame_support::pallet]
pub mod pallet {
	pub use super::*;
	#[derive(TypeInfo, Default, Encode, Decode, Clone)]
	#[scale_info(skip_type_params(T))]
	// Create struct kitty
	pub struct Kitties<T: Config> {
		dna: Vec<u8>,
		owner: T::AccountId,
		price: u32,
		gender: Gender,
	}

	pub type Dna = Vec<u8>;

	#[derive(TypeInfo, Encode ,Decode, Debug, Clone)]
	pub enum Gender {
		Male,
		Female,
	}

	impl Default for Gender{
		fn default()-> Self{
			Gender::Male
		}
	}

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn kitty_qty)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	pub type KittyQty<T> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn kitty)]
	pub(super) type Kitty<T: Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, Kitties<T>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn own)]
	pub(super) type Own<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, Vec<Dna>, OptionQuery>;


	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		KittyStored(Vec<u8>, u32),

		KittyTransfered(Vec<u8>, T::AccountId)
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		// Avoid user send to themselves
		UserNotValid,
		// Avoid same DNA exist
		DnaExist,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn create_kitty(origin: OriginFor<T>, dna: Vec<u8>, price: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who = ensure_signed(origin)?;
			// Ensure no duplicate dna
			ensure!(!<Kitty<T>>::contains_key(&dna), Error::<T>::DnaExist);
			// Get kitty's gender.
			let gender = Self::gen_gender(dna.clone())?;

			// Save new kitty to storage.
			let kitty = Kitties {
				dna: dna.clone(),
				owner: who.clone(),
				price: price,
				gender: gender,
			};
			<Kitty<T>>::insert(dna.clone(), kitty);
			
			// Update total quantity of kitty in system.
			let mut current_qty = <KittyQty<T>>::get();
			current_qty += 1;
			<KittyQty<T>>::put(current_qty);

			// Update kitty to owner storage
			<Own<T>>::append(who, dna.clone());

			// Emit an event.
			Self::deposit_event(Event::KittyStored(dna, price));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		// Transfer kitty.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn transfer_kitty(origin: OriginFor<T>, dna: Vec<u8>, new_owner: T::AccountId) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let kitty_data = <Kitty<T>>::get(dna.clone());
			ensure!(kitty_data.clone().unwrap().owner == who, Error::<T>::NoneValue);
			ensure!(new_owner != who, Error::<T>::UserNotValid);
			match kitty_data {
				Some(mut kitty) => {/*Save new owner for kitty to kitty storage */
									kitty.owner = new_owner.clone();
									<Kitty<T>>::insert(dna.clone(), kitty);

									// Remove kitty from old owner in own storage
									let mut old_balance = <Own<T>>::get(&who).unwrap();
									let a = old_balance.clone();
									for element in a.iter() {
										if element == &dna {
											let index = a.iter().position(|x| *x == dna.clone()).unwrap();
											old_balance.swap_remove(index);
										}
									}
									<Own<T>>::insert(who, old_balance);

									// Update new owner for kitty in own storage
									<Own<T>>::append(&new_owner, &dna);

									// Emit an event.
									Self::deposit_event(Event::KittyTransfered(dna, new_owner));
									// Return a successful DispatchResultWithPostInfo
									Ok(())
				},
				None => Err(DispatchError::Other("There is no kitty")),
			}
		}
	}
}

impl<T> Pallet<T> {
	fn gen_gender(dna: Vec<u8>) -> Result<Gender,Error<T>>{
		let mut res = Gender::Male;
		if dna.len() % 2 !=0 {
			res = Gender::Female;
		}
		Ok(res)
	}
}