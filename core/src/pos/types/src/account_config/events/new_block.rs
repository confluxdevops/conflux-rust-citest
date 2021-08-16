// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: Apache-2.0

// Copyright 2021 Conflux Foundation. All rights reserved.
// Conflux is free software and distributed under GNU General Public License.
// See http://www.gnu.org/licenses/

use crate::account_address::AccountAddress;
use anyhow::Result;
use move_core_types::move_resource::MoveResource;
use serde::{Deserialize, Serialize};

/// Struct that represents a NewBlockEvent.
#[derive(Debug, Serialize, Deserialize)]
pub struct NewBlockEvent {
    round: u64,
    proposer: AccountAddress,
    previous_block_votes: Vec<AccountAddress>,
    time_micro_seconds: u64,
}

impl NewBlockEvent {
    pub fn round(&self) -> u64 { self.round }

    pub fn proposer(&self) -> AccountAddress { self.proposer }

    pub fn proposed_time(&self) -> u64 { self.time_micro_seconds }

    pub fn try_from_bytes(bytes: &[u8]) -> Result<Self> {
        bcs::from_bytes(bytes).map_err(Into::into)
    }

    #[cfg(any(test, feature = "fuzzing"))]
    pub fn new(
        round: u64, proposer: AccountAddress,
        previous_block_votes: Vec<AccountAddress>, time_micro_seconds: u64,
    ) -> Self
    {
        Self {
            round,
            proposer,
            previous_block_votes,
            time_micro_seconds,
        }
    }
}

impl MoveResource for NewBlockEvent {
    const MODULE_NAME: &'static str = "DiemBlock";
    const STRUCT_NAME: &'static str = "NewBlockEvent";
}
