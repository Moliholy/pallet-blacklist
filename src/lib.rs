#![cfg_attr(not(feature = "std"), no_std)]

//! # Blacklist Pallet
//!
//! - [`Config`]
//!
//! ## Overview
//!
//! Allow some configurable origin: [`Config::BlacklistingOrigin`] to blacklist some accounts.
//! Blacklisted accounts are not allowed to perform certain actions.

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    use super::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Required origin for whitelisting an account.
        type BlacklistingOrigin: EnsureOrigin<Self::RuntimeOrigin>;

        /// The weight information for this pallet.
        type WeightInfo: WeightInfo;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::event]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config> {
        AccountBlacklisted { account: T::AccountId },
        BlacklistedAccountRemoved { account: T::AccountId },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// The account was not blacklisted.
        AccountIsNotBlacklisted,
        /// The account was already blacklisted; No-Op.
        AccountAlreadyBlacklisted,
    }

    #[pallet::storage]
    #[pallet::getter(fn blacklist)]
    pub type BlacklistedAccount<T: Config> =
        StorageMap<_, Twox64Concat, T::AccountId, (), OptionQuery>;

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::blacklist_account())]
        pub fn blacklist_account(origin: OriginFor<T>, account: T::AccountId) -> DispatchResult {
            T::BlacklistingOrigin::ensure_origin(origin)?;

            ensure!(
                !BlacklistedAccount::<T>::contains_key(&account),
                Error::<T>::AccountAlreadyBlacklisted,
            );

            BlacklistedAccount::<T>::insert(&account, ());

            Self::deposit_event(Event::<T>::AccountBlacklisted { account });

            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::remove_blacklisted_account())]
        pub fn remove_blacklisted_account(
            origin: OriginFor<T>,
            account: T::AccountId,
        ) -> DispatchResult {
            T::BlacklistingOrigin::ensure_origin(origin)?;

            BlacklistedAccount::<T>::take(&account).ok_or(Error::<T>::AccountIsNotBlacklisted)?;

            Self::deposit_event(Event::<T>::BlacklistedAccountRemoved { account });

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn is_account_blacklisted(account: &T::AccountId) -> bool {
            BlacklistedAccount::<T>::take(account).is_some()
        }
    }

    impl<T: Config> EnsureOrigin<T::AccountId> for Pallet<T> {
        type Success = ();

        fn try_origin(o: T::AccountId) -> Result<Self::Success, T::AccountId> {
            match Self::is_account_blacklisted(&o) {
                false => Ok(()),
                true => Err(o),
            }
        }

        #[cfg(feature = "runtime-benchmarks")]
        fn try_successful_origin() -> Result<T::AccountId, ()> {
            Err(())
        }
    }
}
