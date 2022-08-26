// Copyright 2022 Parity Technologies (UK) Ltd.
// This file is part of Parity Bridges Common.

// Parity Bridges Common is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity Bridges Common is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity Bridges Common.  If not, see <http://www.gnu.org/licenses/>.

//! Wococo-to-Rococo bridge hubs headers sync entrypoint.

use crate::cli::bridge::{CliBridgeBase, RelayToRelayHeadersCliBridge};
use substrate_relay_helper::finality::{
	engine::Grandpa as GrandpaFinalityEngine, SubstrateFinalitySyncPipeline,
};

/// Description of Wococo -> Rococo finalized headers bridge.
#[derive(Clone, Debug)]
pub struct BridgeHubWococoFinalityToBridgeHubRococo;

substrate_relay_helper::generate_mocked_submit_finality_proof_call_builder!(
	BridgeHubWococoFinalityToBridgeHubRococo,
	BridgeHubWococoFinalityToBridgeHubRococoCallBuilder,
	relay_bridge_hub_rococo_client::runtime::Call::BridgeGrandpaWococo,
	relay_bridge_hub_rococo_client::runtime::BridgeGrandpaWococoCall::submit_finality_proof
);

impl SubstrateFinalitySyncPipeline for BridgeHubWococoFinalityToBridgeHubRococo {
	type SourceChain = relay_bridge_hub_wococo_client::BridgeHubWococo;
	type TargetChain = relay_bridge_hub_rococo_client::BridgeHubRococo;

	type FinalityEngine = GrandpaFinalityEngine<Self::SourceChain>;
	type SubmitFinalityProofCallBuilder = BridgeHubWococoFinalityToBridgeHubRococoCallBuilder;
	type TransactionSignScheme = relay_bridge_hub_rococo_client::BridgeHubRococo;
}

/// BridgeHub `Wococo` to BridgeHub `Rococo` bridge definition.
pub struct BridgeHubWococoToBridgeHubRococoCliBridge {}

impl CliBridgeBase for BridgeHubWococoToBridgeHubRococoCliBridge {
	type Source = relay_bridge_hub_wococo_client::BridgeHubWococo;
	type Target = relay_bridge_hub_rococo_client::BridgeHubRococo;
}

impl RelayToRelayHeadersCliBridge for BridgeHubWococoToBridgeHubRococoCliBridge {
	type Finality = BridgeHubWococoFinalityToBridgeHubRococo;
}
