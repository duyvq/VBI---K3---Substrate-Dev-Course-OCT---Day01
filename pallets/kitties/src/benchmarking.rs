//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as Kitty;
use frame_benchmarking::{benchmarks, whitelisted_caller, account};
use frame_system::RawOrigin;
use frame_benchmarking::vec;
use frame_support::traits::Randomness;

benchmarks! { 
	// tên của benchmark
	create_kitty {
		// khởi tạo các tham số cho extrinsic benchmark
		let dnas : Vec<u8> = b"duy".to_vec();

		let caller: T::AccountId = whitelisted_caller();
	}: create_kitty (RawOrigin::Signed(caller), dnas)

	// kiểm tra lại trạng thái storage khi thực hiện extrinsic xem đúng chưa 
	verify {
		assert_eq!(KittyId::<T>::get(), 1);
	}

	transfer {
		// Khởi tạo các tham số để tạo kitties
		let dnas = b"duy".to_vec();
		let from: T::AccountId = whitelisted_caller();
		Kitty::<T>::create_kitty(<T>::Origin::from(frame_system::RawOrigin::Signed(from.clone())), dnas);
		
		// Khởi tạo các tham số để transfer kitty
		let dna_hash: T::Hash = Kitty::<T>::gen_dna();
		let to: T::AccountId = account("BOB", 2, 2);
	}: transfer (RawOrigin::Signed(from.clone()), to.clone(), dna_hash.clone())

	verify {
		assert_eq!(KittyId::<T>::get(), 1);
		assert_eq!(KittiesOwned::<T>::get(from).len(), 0);
		assert_eq!(KittiesOwned::<T>::get(&to).len(), 1);
		assert_eq!(Kitties::<T>::get(dna_hash).unwrap().owner, to);
		// assert_eq!(Kitties::<T>::get(dna_hash), Some (pallet::Kitty{dna: _, price: _ , gender, owner: to, created_date}));
	}
 
	// thực hiện benchmark với mock runtime, storage ban đầu.
	impl_benchmark_test_suite!(Kitty, crate::mock::new_test_ext(), crate::mock::Test);
}
