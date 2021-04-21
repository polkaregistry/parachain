// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of Polkaregistry.
//
// Copyright (c) 2021 Wei Tang.
//
// Polkaregistry is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Polkaregistry is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Polkaregistry. If not, see <http://www.gnu.org/licenses/>.

//! Auxillary struct/enums for Polkaregistry runtime.
//! Taken from Statemint and adapted for Polkaregistry.

use frame_support::traits::{Currency, OnUnbalanced};

pub type NegativeImbalance<T> = <pallet_balances::Pallet<T> as Currency<<T as frame_system::Config>::AccountId>>::NegativeImbalance;

pub struct DealWithFees<R>(sp_std::marker::PhantomData<R>);
impl<R> OnUnbalanced<NegativeImbalance<R>> for DealWithFees<R>
where
	R: pallet_balances::Config,
	<R as frame_system::Config>::AccountId: From<polkadot_primitives::v1::AccountId>,
	<R as frame_system::Config>::AccountId: Into<polkadot_primitives::v1::AccountId>,
	<R as frame_system::Config>::Event: From<pallet_balances::Event<R>>,
{
	fn on_unbalanceds<B>(mut _fees_then_tips: impl Iterator<Item = NegativeImbalance<R>>) {
		// Do nothing for now.
	}
}
