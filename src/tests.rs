use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};
use sp_runtime::traits::BadOrigin;

#[test]
fn it_successfully_inserts_blacklisted_accounts() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        // At the beginning there is no account blacklisted
        assert_eq!(BlacklistModule::blacklist(42), None);
        // we insert it now
        assert_ok!(BlacklistModule::blacklist_account(
            RuntimeOrigin::root(),
            42
        ));
        // check the account is there
        assert_eq!(BlacklistModule::blacklist(42), Some(()));
        assert!(BlacklistModule::is_account_blacklisted(&42));
        System::assert_last_event(Event::AccountBlacklisted { account: 42 }.into());
    });
}

#[test]
fn only_root_can_add_to_blacklist() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        // if Root is not performing the operation it should fail
        assert_noop!(
            BlacklistModule::blacklist_account(RuntimeOrigin::signed(1), 42),
            BadOrigin
        );
    });
}

#[test]
fn error_if_inserting_again() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(BlacklistModule::blacklist_account(
            RuntimeOrigin::root(),
            42
        ));

        // try and insert again the same value
        assert_noop!(
            BlacklistModule::blacklist_account(RuntimeOrigin::root(), 42),
            Error::<Test>::AccountAlreadyBlacklisted
        );
    });
}

#[test]
fn it_successfully_removes_blacklisted_accounts() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        // insert the blacklisted account
        assert_ok!(BlacklistModule::blacklist_account(
            RuntimeOrigin::root(),
            42
        ));
        // remove the account
        assert_eq!(
            BlacklistModule::remove_blacklisted_account(RuntimeOrigin::root(), 42),
            Ok(())
        );
        System::assert_last_event(Event::BlacklistedAccountRemoved { account: 42 }.into());
        // check it is no longer there
        assert_eq!(BlacklistModule::blacklist(42), None);
        assert!(!BlacklistModule::is_account_blacklisted(&42));
    });
}

#[test]
fn only_root_can_remove_from_blacklist() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(BlacklistModule::blacklist_account(
            RuntimeOrigin::root(),
            42
        ));
        // if Root is not performing the operation it should fail
        assert_noop!(
            BlacklistModule::remove_blacklisted_account(RuntimeOrigin::signed(1), 42),
            BadOrigin
        );
    });
}

#[test]
fn error_if_account_is_not_blacklisted() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        // cannot remove the account from the blacklist if it is not there
        assert_noop!(
            BlacklistModule::remove_blacklisted_account(RuntimeOrigin::root(), 42),
            Error::<Test>::AccountIsNotBlacklisted
        );
    });
}
