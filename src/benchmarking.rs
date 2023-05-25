//! Benchmarking setup for pallet-template
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as Blacklist;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn blacklist_account() {
        let value: T::AccountId = account::<T::AccountId>("blacklist_caller", 0, 0);
        #[extrinsic_call]
        blacklist_account(RawOrigin::Root, value.clone());

        assert_eq!(BlacklistedAccount::<T>::get(value), Some(()));
    }

    #[benchmark]
    fn remove_blacklisted_account() {
        let value: T::AccountId = account::<T::AccountId>("blacklist_caller", 0, 0);
        BlacklistedAccount::<T>::insert(&value, ());
        #[extrinsic_call]
        remove_blacklisted_account(RawOrigin::Root, value.clone());

        assert_eq!(BlacklistedAccount::<T>::get(value), Some(()));
    }

    impl_benchmark_test_suite!(Blacklist, crate::mock::new_test_ext(), crate::mock::Test);
}