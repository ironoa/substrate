// This file is part of Substrate.

// Copyright (C) 2017-2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Proof-of-Personhood system.

#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "128"]

use codec::{MaxEncodedLen, FullCodec};
use scale_info::TypeInfo;
use sp_arithmetic::traits::{Zero, Saturating};
use sp_runtime::traits::{AtLeast32BitUnsigned, Convert};
use sp_std::{marker::PhantomData, fmt::Debug, prelude::*};

use frame_support::{
	dispatch::DispatchResultWithPostInfo, ensure, traits::RankedMembers,
};

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;

pub use pallet::*;
pub use weights::WeightInfo;

/// Payroll cycle.
pub type Cycle = u32;

// Can be implemented by Pot pallet with a fixed Currency impl, but can also be implemented with
// XCM/MultiAsset and made generic over assets.
pub trait Pay {
	/// The type by which we measure units of the currency in which we make payments.
	type Balance: AtLeast32BitUnsigned + FullCodec + MaxEncodedLen + TypeInfo;
	/// The type by which we identify the individuals to whom a payment may be made.
	type AccountId;
	/// An identifier given to an individual payment.
	type Id: FullCodec + MaxEncodedLen + TypeInfo + Clone + Eq + PartialEq + Debug;
	/// The amount of currency with which we have to make payments in this period. It may be a fixed
	/// value or reduce as calls to `pay` are made. It should be called once prior to the series of
	/// payments to determine the overall budget and then not again until the next series of
	/// payments are to be made.
	fn budget() -> Self::Balance;
	/// Make a payment and return an identifier for later evaluation of success in some off-chain
	/// mechanism (likely an event, but possibly not on this chain).
	fn pay(who: &Self::AccountId, amount: Self::Balance) -> Result<Self::Id, ()>;
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{pallet_prelude::*, dispatch::Pays};
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T, I = ()>(PhantomData<(T, I)>);

	#[pallet::config]
	pub trait Config<I: 'static = ()>: frame_system::Config {
		/// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;

		/// The runtime event type.
		type RuntimeEvent: From<Event<Self, I>>
			+ IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Means by which we can make payments to accounts. This also defines the currency and the
		/// balance which we use to denote that currency.
		type Paymaster: Pay<AccountId = <Self as frame_system::Config>::AccountId>;

		/// The current membership of payees.
		type Members: RankedMembers<AccountId = <Self as frame_system::Config>::AccountId>;

		/// The maximum payout to be made for a single period to an active member of the given rank.
		type ActiveSalaryForRank: Convert<<Self::Members as RankedMembers>::Rank, <Self::Paymaster as Pay>::Balance>;

		/// The number of blocks between sequential payout cycles.
		#[pallet::constant]
		type CyclePeriod: Get<Self::BlockNumber>;
	}

	pub type CycleIndexOf<T> = <T as frame_system::Config>::BlockNumber;

	/// The current payout cycle, the block nunber at which it started and the remaining balance in
	/// this cycle's budget.
	#[pallet::storage]
	pub(super) type Status<T: Config<I>, I: 'static = ()> =
		StorageValue<_, (CycleIndexOf<T>, T::BlockNumber, <T::Paymaster as Pay>::Balance), OptionQuery>;

	/// The most recent cycle which was paid to a member. None implies that a member has not yet
	/// been paid.
	#[pallet::storage]
	pub(super) type LastClaim<T: Config<I>, I: 'static = ()> =
		StorageMap<_, Twox64Concat, T::AccountId, CycleIndexOf<T>, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config<I>, I: 'static = ()> {
		/// A member is inducted into the payroll.
		Inducted { who: T::AccountId },
		/// A payment happened.
		Paid { who: T::AccountId, id: <T::Paymaster as Pay>::Id },
	}

	#[pallet::error]
	pub enum Error<T, I = ()> {
		/// The account is not a ranked member.
		NotMember,
		// The account is not yet inducted into the system.
		NotInducted,
		/// The member does not have a current valid claim.
		NoClaim,
		/// The member's claim is zero.
		ClaimZero,
		/// Cycle is not yet over.
		NotYet,
		/// The payout cycles have not yet started.
		NotStarted,
		/// There is no budget left for the payout.
		Bankrupt,
		/// There was some issue with the mechanism of payment.
		PayError,
	}

	#[pallet::call]
	impl<T: Config<I>, I: 'static> Pallet<T, I> {
		/// Induct oneself into the payout system.
		#[pallet::weight(T::WeightInfo::add_member())]
		#[pallet::call_index(0)]
		pub fn induct(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			let _ = T::Members::rank_of(&who).ok_or(Error::<T, I>::NotMember)?;
			let last_payout = Status::<T, I>::get().map_or(Zero::zero(), |x| x.1);
			LastClaim::<T, I>::insert(&who, last_payout);
			Self::deposit_event(Event::<T, I>::Inducted { who });
			Ok(Pays::No.into())
		}

		/// Request a payout.
		///
		/// - `origin`: A `Signed` origin of an account which is a member of `Members`.
		#[pallet::weight(T::WeightInfo::add_member())]
		#[pallet::call_index(1)]
		pub fn payout(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			let rank = T::Members::rank_of(&who).ok_or(Error::<T, I>::NotMember)?;
			let payout = T::ActiveSalaryForRank::convert(rank);
			ensure!(!payout.is_zero(), Error::<T, I>::ClaimZero);
			let last_claim = LastClaim::<T, I>::get(&who).ok_or(Error::<T, I>::NotInducted)?;
			let (_, last_payout, budget) = Status::<T, I>::get().ok_or(Error::<T, I>::NotStarted)?;
			ensure!(last_claim < last_payout, Error::<T, I>::NoClaim);
			ensure!(payout <= budget, Error::<T, I>::Bankrupt);

			let id = T::Paymaster::pay(&who, payout).map_err(|()| Error::<T, I>::PayError)?;

			LastClaim::<T, I>::insert(&who, last_payout);
			Self::deposit_event(Event::<T, I>::Paid { who, id });
			Ok(Pays::No.into())
		}

		/// Move to next payout cycle, assuming that the present block is now within that cycle.
		///
		/// - `origin`: A `Signed` origin of an account which is a member of `Members`.
		#[pallet::weight(T::WeightInfo::add_member())]
		#[pallet::call_index(2)]
		pub fn next_cycle(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let _ = ensure_signed(origin)?;
			let now = frame_system::Pallet::<T>::block_number();
			let (mut cycle_index, mut cycle_start, _) = Status::<T, I>::get()
				.ok_or(Error::<T, I>::NoClaim)?;
			cycle_start.saturating_accrue(T::CyclePeriod::get());
			ensure!(now >= cycle_start, Error::<T, I>::NotYet);
			cycle_index.saturating_inc();
			let budget = T::Paymaster::budget();
			Status::<T, I>::put((cycle_index, cycle_start, budget));
			Ok(Pays::No.into())
		}
	}

	impl<T: Config<I>, I: 'static> Pallet<T, I> {}
}
