// Copyright 2017-2022 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.
//! Autogenerated weights for `runtime_parachains::hrmp`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-11-16, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `bm6`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("westend-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=westend-dev
// --steps=50
// --repeat=20
// --pallet=runtime_parachains::hrmp
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/westend/src/weights/runtime_parachains_hrmp.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `runtime_parachains::hrmp`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> runtime_parachains::hrmp::WeightInfo for WeightInfo<T> {
	// Storage: Paras ParaLifecycles (r:2 w:0)
	// Storage: Hrmp HrmpOpenChannelRequests (r:1 w:1)
	// Storage: Hrmp HrmpChannels (r:1 w:0)
	// Storage: Hrmp HrmpEgressChannelsIndex (r:1 w:0)
	// Storage: Hrmp HrmpOpenChannelRequestCount (r:1 w:1)
	// Storage: Hrmp HrmpOpenChannelRequestsList (r:1 w:1)
	// Storage: Dmp DownwardMessageQueueHeads (r:1 w:1)
	// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	fn hrmp_init_open_channel() -> Weight {
		// Minimum execution time: 42_236 nanoseconds.
		Weight::from_ref_time(42_643_000 as u64)
			.saturating_add(T::DbWeight::get().reads(9 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	// Storage: Hrmp HrmpOpenChannelRequests (r:1 w:1)
	// Storage: Paras ParaLifecycles (r:1 w:0)
	// Storage: Hrmp HrmpIngressChannelsIndex (r:1 w:0)
	// Storage: Hrmp HrmpAcceptedChannelRequestCount (r:1 w:1)
	// Storage: Dmp DownwardMessageQueueHeads (r:1 w:1)
	// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	fn hrmp_accept_open_channel() -> Weight {
		// Minimum execution time: 42_348 nanoseconds.
		Weight::from_ref_time(45_358_000 as u64)
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: Hrmp HrmpChannels (r:1 w:0)
	// Storage: Hrmp HrmpCloseChannelRequests (r:1 w:1)
	// Storage: Hrmp HrmpCloseChannelRequestsList (r:1 w:1)
	// Storage: Dmp DownwardMessageQueueHeads (r:1 w:1)
	// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	fn hrmp_close_channel() -> Weight {
		// Minimum execution time: 44_696 nanoseconds.
		Weight::from_ref_time(46_077_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: Hrmp HrmpIngressChannelsIndex (r:128 w:127)
	// Storage: Hrmp HrmpEgressChannelsIndex (r:1 w:1)
	// Storage: Hrmp HrmpChannels (r:127 w:127)
	// Storage: Hrmp HrmpAcceptedChannelRequestCount (r:0 w:1)
	// Storage: Hrmp HrmpChannelContents (r:0 w:127)
	// Storage: Hrmp HrmpOpenChannelRequestCount (r:0 w:1)
	/// The range of component `i` is `[0, 127]`.
	/// The range of component `e` is `[0, 127]`.
	fn force_clean_hrmp(i: u32, e: u32, ) -> Weight {
		// Minimum execution time: 867_905 nanoseconds.
		Weight::from_ref_time(871_643_000 as u64)
			// Standard Error: 80_060
			.saturating_add(Weight::from_ref_time(2_666_207 as u64).saturating_mul(i as u64))
			// Standard Error: 80_060
			.saturating_add(Weight::from_ref_time(2_737_378 as u64).saturating_mul(e as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().reads((2 as u64).saturating_mul(i as u64)))
			.saturating_add(T::DbWeight::get().reads((2 as u64).saturating_mul(e as u64)))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
			.saturating_add(T::DbWeight::get().writes((3 as u64).saturating_mul(i as u64)))
			.saturating_add(T::DbWeight::get().writes((3 as u64).saturating_mul(e as u64)))
	}
	// Storage: Hrmp HrmpOpenChannelRequestsList (r:1 w:0)
	// Storage: Hrmp HrmpOpenChannelRequests (r:2 w:2)
	// Storage: Paras ParaLifecycles (r:4 w:0)
	// Storage: Hrmp HrmpIngressChannelsIndex (r:2 w:2)
	// Storage: Hrmp HrmpEgressChannelsIndex (r:2 w:2)
	// Storage: Hrmp HrmpOpenChannelRequestCount (r:2 w:2)
	// Storage: Hrmp HrmpAcceptedChannelRequestCount (r:2 w:2)
	// Storage: Hrmp HrmpChannels (r:0 w:2)
	/// The range of component `c` is `[0, 128]`.
	fn force_process_hrmp_open(c: u32, ) -> Weight {
		// Minimum execution time: 10_127 nanoseconds.
		Weight::from_ref_time(465_403 as u64)
			// Standard Error: 13_887
			.saturating_add(Weight::from_ref_time(15_736_845 as u64).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().reads((7 as u64).saturating_mul(c as u64)))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
			.saturating_add(T::DbWeight::get().writes((6 as u64).saturating_mul(c as u64)))
	}
	// Storage: Hrmp HrmpCloseChannelRequestsList (r:1 w:0)
	// Storage: Hrmp HrmpChannels (r:2 w:2)
	// Storage: Hrmp HrmpEgressChannelsIndex (r:2 w:2)
	// Storage: Hrmp HrmpIngressChannelsIndex (r:2 w:2)
	// Storage: Hrmp HrmpCloseChannelRequests (r:0 w:2)
	// Storage: Hrmp HrmpChannelContents (r:0 w:2)
	/// The range of component `c` is `[0, 128]`.
	fn force_process_hrmp_close(c: u32, ) -> Weight {
		// Minimum execution time: 6_473 nanoseconds.
		Weight::from_ref_time(335_647 as u64)
			// Standard Error: 11_098
			.saturating_add(Weight::from_ref_time(9_684_287 as u64).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().reads((3 as u64).saturating_mul(c as u64)))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
			.saturating_add(T::DbWeight::get().writes((5 as u64).saturating_mul(c as u64)))
	}
	// Storage: Hrmp HrmpOpenChannelRequestsList (r:1 w:1)
	// Storage: Hrmp HrmpOpenChannelRequests (r:1 w:1)
	// Storage: Hrmp HrmpOpenChannelRequestCount (r:1 w:1)
	/// The range of component `c` is `[0, 128]`.
	fn hrmp_cancel_open_request(c: u32, ) -> Weight {
		// Minimum execution time: 25_408 nanoseconds.
		Weight::from_ref_time(31_410_214 as u64)
			// Standard Error: 1_341
			.saturating_add(Weight::from_ref_time(97_088 as u64).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Hrmp HrmpOpenChannelRequestsList (r:1 w:1)
	// Storage: Hrmp HrmpOpenChannelRequests (r:2 w:2)
	/// The range of component `c` is `[0, 128]`.
	fn clean_open_channel_requests(c: u32, ) -> Weight {
		// Minimum execution time: 4_587 nanoseconds.
		Weight::from_ref_time(3_141_418 as u64)
			// Standard Error: 4_136
			.saturating_add(Weight::from_ref_time(2_571_806 as u64).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(c as u64)))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(c as u64)))
	}
	// Storage: Paras ParaLifecycles (r:2 w:0)
	// Storage: Hrmp HrmpOpenChannelRequests (r:1 w:1)
	// Storage: Hrmp HrmpChannels (r:1 w:0)
	// Storage: Hrmp HrmpEgressChannelsIndex (r:1 w:0)
	// Storage: Hrmp HrmpOpenChannelRequestCount (r:1 w:1)
	// Storage: Hrmp HrmpOpenChannelRequestsList (r:1 w:1)
	// Storage: Dmp DownwardMessageQueueHeads (r:2 w:2)
	// Storage: Dmp DownwardMessageQueues (r:2 w:2)
	// Storage: Hrmp HrmpIngressChannelsIndex (r:1 w:0)
	// Storage: Hrmp HrmpAcceptedChannelRequestCount (r:1 w:1)
	fn force_open_hrmp_channel() -> Weight {
		// Minimum execution time: 52_246 nanoseconds.
		Weight::from_ref_time(52_759_000 as u64)
			.saturating_add(T::DbWeight::get().reads(13 as u64))
			.saturating_add(T::DbWeight::get().writes(8 as u64))
	}
}
