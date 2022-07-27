use crate::{mock::*, Error, Students, Gender::Male};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_creating_student_age_greater_20() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(Demo::create_student(Origin::signed(123), vec![100, 117, 121, 118, 113], 30));
		// Read pallet storage and assert an expected result.
		assert_eq!(Demo::student_id(), 1);
		// Read pallet storage and assert an expected result.
		assert_eq!(Demo::student(1), Some(Students {	name: b"duyvq".to_vec(),
										age: 30,
										gender: Male,
										account: 123,
		}));
	});
}

#[test]
fn correct_error_for_student_age_equals_or_less_20() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(Demo::create_student(Origin::signed(1), "duyqv".as_bytes().to_vec(), 20), Error::<Test>::TooYoung);
	});
}
