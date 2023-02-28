// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
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

//! Autogenerated weights for pallet_message_queue
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-24, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bm3`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/production/substrate
// benchmark
// pallet
// --steps=50
// --repeat=20
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --json-file=/var/lib/gitlab-runner/builds/zyw4fam_/0/parity/mirrors/substrate/.git/.artifacts/bench.json
// --pallet=pallet_message_queue
// --chain=dev
// --header=./HEADER-APACHE2
// --output=./frame/message-queue/src/weights.rs
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_message_queue.
pub trait WeightInfo {
	fn ready_ring_knit() -> Weight;
	fn ready_ring_unknit() -> Weight;
	fn service_queue_base() -> Weight;
	fn service_page_base_completion() -> Weight;
	fn service_page_base_no_completion() -> Weight;
	fn service_page_item() -> Weight;
	fn bump_service_head() -> Weight;
	fn reap_page() -> Weight;
	fn execute_overweight_page_removed() -> Weight;
	fn execute_overweight_page_updated() -> Weight;
}

/// Weights for pallet_message_queue using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: MessageQueue ServiceHead (r:1 w:0)
	/// Proof: MessageQueue ServiceHead (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: MessageQueue BookStateFor (r:2 w:2)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	fn ready_ring_knit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `829`
		//  Estimated: `5547`
		// Minimum execution time: 15_241 nanoseconds.
		Weight::from_ref_time(15_603_000)
			.saturating_add(Weight::from_proof_size(5547))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: MessageQueue BookStateFor (r:2 w:2)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: MessageQueue ServiceHead (r:1 w:1)
	/// Proof: MessageQueue ServiceHead (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn ready_ring_unknit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `829`
		//  Estimated: `5547`
		// Minimum execution time: 14_652 nanoseconds.
		Weight::from_ref_time(14_983_000)
			.saturating_add(Weight::from_proof_size(5547))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: MessageQueue BookStateFor (r:1 w:1)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	fn service_queue_base() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `576`
		//  Estimated: `2524`
		// Minimum execution time: 5_750 nanoseconds.
		Weight::from_ref_time(6_003_000)
			.saturating_add(Weight::from_proof_size(2524))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: MessageQueue Pages (r:1 w:1)
	/// Proof: MessageQueue Pages (max_values: None, max_size: Some(65584), added: 68059, mode: MaxEncodedLen)
	fn service_page_base_completion() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `647`
		//  Estimated: `68059`
		// Minimum execution time: 8_257 nanoseconds.
		Weight::from_ref_time(8_506_000)
			.saturating_add(Weight::from_proof_size(68059))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: MessageQueue Pages (r:1 w:1)
	/// Proof: MessageQueue Pages (max_values: None, max_size: Some(65584), added: 68059, mode: MaxEncodedLen)
	fn service_page_base_no_completion() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `647`
		//  Estimated: `68059`
		// Minimum execution time: 8_422 nanoseconds.
		Weight::from_ref_time(8_589_000)
			.saturating_add(Weight::from_proof_size(68059))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn service_page_item() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `972`
		//  Estimated: `0`
		// Minimum execution time: 81_929 nanoseconds.
		Weight::from_ref_time(82_375_000)
			.saturating_add(Weight::from_proof_size(0))
	}
	/// Storage: MessageQueue ServiceHead (r:1 w:1)
	/// Proof: MessageQueue ServiceHead (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: MessageQueue BookStateFor (r:1 w:0)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	fn bump_service_head() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `706`
		//  Estimated: `3023`
		// Minimum execution time: 8_992 nanoseconds.
		Weight::from_ref_time(9_200_000)
			.saturating_add(Weight::from_proof_size(3023))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: MessageQueue BookStateFor (r:1 w:1)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: MessageQueue Pages (r:1 w:1)
	/// Proof: MessageQueue Pages (max_values: None, max_size: Some(65584), added: 68059, mode: MaxEncodedLen)
	fn reap_page() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `66825`
		//  Estimated: `70583`
		// Minimum execution time: 68_292 nanoseconds.
		Weight::from_ref_time(69_108_000)
			.saturating_add(Weight::from_proof_size(70583))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: MessageQueue BookStateFor (r:1 w:1)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: MessageQueue Pages (r:1 w:1)
	/// Proof: MessageQueue Pages (max_values: None, max_size: Some(65584), added: 68059, mode: MaxEncodedLen)
	fn execute_overweight_page_removed() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `66825`
		//  Estimated: `70583`
		// Minimum execution time: 83_855 nanoseconds.
		Weight::from_ref_time(84_946_000)
			.saturating_add(Weight::from_proof_size(70583))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: MessageQueue BookStateFor (r:1 w:1)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: MessageQueue Pages (r:1 w:1)
	/// Proof: MessageQueue Pages (max_values: None, max_size: Some(65584), added: 68059, mode: MaxEncodedLen)
	fn execute_overweight_page_updated() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `66825`
		//  Estimated: `70583`
		// Minimum execution time: 96_997 nanoseconds.
		Weight::from_ref_time(98_668_000)
			.saturating_add(Weight::from_proof_size(70583))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: MessageQueue ServiceHead (r:1 w:0)
	/// Proof: MessageQueue ServiceHead (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: MessageQueue BookStateFor (r:2 w:2)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	fn ready_ring_knit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `829`
		//  Estimated: `5547`
		// Minimum execution time: 15_241 nanoseconds.
		Weight::from_ref_time(15_603_000)
			.saturating_add(Weight::from_proof_size(5547))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: MessageQueue BookStateFor (r:2 w:2)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: MessageQueue ServiceHead (r:1 w:1)
	/// Proof: MessageQueue ServiceHead (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn ready_ring_unknit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `829`
		//  Estimated: `5547`
		// Minimum execution time: 14_652 nanoseconds.
		Weight::from_ref_time(14_983_000)
			.saturating_add(Weight::from_proof_size(5547))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: MessageQueue BookStateFor (r:1 w:1)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	fn service_queue_base() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `576`
		//  Estimated: `2524`
		// Minimum execution time: 5_750 nanoseconds.
		Weight::from_ref_time(6_003_000)
			.saturating_add(Weight::from_proof_size(2524))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: MessageQueue Pages (r:1 w:1)
	/// Proof: MessageQueue Pages (max_values: None, max_size: Some(65584), added: 68059, mode: MaxEncodedLen)
	fn service_page_base_completion() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `647`
		//  Estimated: `68059`
		// Minimum execution time: 8_257 nanoseconds.
		Weight::from_ref_time(8_506_000)
			.saturating_add(Weight::from_proof_size(68059))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: MessageQueue Pages (r:1 w:1)
	/// Proof: MessageQueue Pages (max_values: None, max_size: Some(65584), added: 68059, mode: MaxEncodedLen)
	fn service_page_base_no_completion() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `647`
		//  Estimated: `68059`
		// Minimum execution time: 8_422 nanoseconds.
		Weight::from_ref_time(8_589_000)
			.saturating_add(Weight::from_proof_size(68059))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn service_page_item() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `972`
		//  Estimated: `0`
		// Minimum execution time: 81_929 nanoseconds.
		Weight::from_ref_time(82_375_000)
			.saturating_add(Weight::from_proof_size(0))
	}
	/// Storage: MessageQueue ServiceHead (r:1 w:1)
	/// Proof: MessageQueue ServiceHead (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: MessageQueue BookStateFor (r:1 w:0)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	fn bump_service_head() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `706`
		//  Estimated: `3023`
		// Minimum execution time: 8_992 nanoseconds.
		Weight::from_ref_time(9_200_000)
			.saturating_add(Weight::from_proof_size(3023))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: MessageQueue BookStateFor (r:1 w:1)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: MessageQueue Pages (r:1 w:1)
	/// Proof: MessageQueue Pages (max_values: None, max_size: Some(65584), added: 68059, mode: MaxEncodedLen)
	fn reap_page() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `66825`
		//  Estimated: `70583`
		// Minimum execution time: 68_292 nanoseconds.
		Weight::from_ref_time(69_108_000)
			.saturating_add(Weight::from_proof_size(70583))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: MessageQueue BookStateFor (r:1 w:1)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: MessageQueue Pages (r:1 w:1)
	/// Proof: MessageQueue Pages (max_values: None, max_size: Some(65584), added: 68059, mode: MaxEncodedLen)
	fn execute_overweight_page_removed() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `66825`
		//  Estimated: `70583`
		// Minimum execution time: 83_855 nanoseconds.
		Weight::from_ref_time(84_946_000)
			.saturating_add(Weight::from_proof_size(70583))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: MessageQueue BookStateFor (r:1 w:1)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: MessageQueue Pages (r:1 w:1)
	/// Proof: MessageQueue Pages (max_values: None, max_size: Some(65584), added: 68059, mode: MaxEncodedLen)
	fn execute_overweight_page_updated() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `66825`
		//  Estimated: `70583`
		// Minimum execution time: 96_997 nanoseconds.
		Weight::from_ref_time(98_668_000)
			.saturating_add(Weight::from_proof_size(70583))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
}
