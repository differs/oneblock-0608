use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};
use super::*;
// use std::vec::Vec;

// #[test]
// fn it_works_for_default_value() {
// 	new_test_ext().execute_with(|| {
// 		// Dispatch a signed extrinsic.
// 		assert_ok!(PoeModule::do_something(Origin::signed(1), 42));
// 		// Read pallet storage and assert an expected result.
// 		assert_eq!(PoeModule::something(), Some(42));
// 	});
// }

// #[test]
// fn correct_error_for_none_value() {
// 	new_test_ext().execute_with(|| {
// 		// Ensure the expected error is thrown when no value is present.
// 		assert_noop!(
// 			TemplateModule::cause_error(Origin::signed(1)),
// 			Error::<Test>::NoneValue
// 		);
// 	});
// }

#[test]

fn create_claim_works() {
	new_test_ext().execute_with(|| {

		let claim: Vec<u8> = vec![0,1];
		assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));

		assert_eq!(
			Proofs::<Test>::get(&claim), 
			(1, frame_system::Pallet::<Test>::block_number()));
	})
}

#[test]

fn create_claim_failed_when_claim_already_exist() {
	new_test_ext().execute_with(|| {

		let claim: Vec<u8> = vec![0,1];
		let _ = (PoeModule::create_claim(Origin::signed(1), claim.clone()));
		// assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
		assert_noop!(PoeModule::create_claim(Origin::signed(1), claim.clone()),
		Error::<Test>::ProofAlreadyClaimed

		);
	})
}

#[test]

fn revoke_claim_works() {
	new_test_ext().execute_with(||{
		let claim: Vec<u8> = vec![0,1];
		let _ = (PoeModule::create_claim(Origin::signed(1), claim.clone()));

		assert_ok!(PoeModule::revoke_claim(Origin::signed(1),claim.clone()));

		assert_eq!(
			Proofs::<Test>::get(&claim),
			(0,0)
		);
	})
}


#[test]

fn revoke_claimfailed_when_claim_already_remove() {
	new_test_ext().execute_with(||{
		let claim: Vec<u8> = vec![0,1];
		let _ = (PoeModule::revoke_claim(Origin::signed(1), claim.clone()));

		assert_noop!(PoeModule::revoke_claim(Origin::signed(1), claim.clone()),
		Error::<Test>::NoSuchProof

		);
	})
}

#[test]

fn transfer_claim_works() {
	new_test_ext().execute_with(|| {
		let claim: Vec<u8> = vec![0,1];
		// let reciver = String::from("0x0");
		let reciver: u64 = 1111;
		let _ = (PoeModule::create_claim(Origin::signed(1), claim.clone()));
		assert_ok!(PoeModule::transfer_claim(Origin::signed(1), claim.clone(), reciver.clone()));
		
	})
}

#[test]

fn transfer_claim_failed_when_NoSuchProof() {
	new_test_ext().execute_with(|| {
		let claim: Vec<u8> = vec![0,1];
		// let reciver = String::from("0x0");
		let reciver: u64 = 1111;
		// let _ = (PoeModule::create_claim(Origin::signed(1), claim.clone()));
		// assert_ok!(PoeModule::transfer_claim(Origin::signed(1), claim.clone(), reciver.clone()));
		assert_noop!(PoeModule::transfer_claim(Origin::signed(1),claim.clone(), reciver.clone()),
		Error::<Test>::NoSuchProof
		);
	})
}


#[test]
/// 创建存征超过长度限制
fn crate_clain_when_proof_is_too_long(){
	new_test_ext().execute_with(|| {
		let proof: Vec<u8> = vec![0,1,2];
		// let proof: Vec<u8> = vec![1];
		assert_noop!(
			PoeModule::create_claim(Origin::signed(1), proof.clone()),
			Error::<Test>::ProofTooLong
		);
	});
}