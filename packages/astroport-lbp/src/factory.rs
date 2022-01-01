use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::asset::{AssetInfo, PairInfo, WeightedAssetInfo};
use cosmwasm_std::Addr;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    /// Pair contract code ID, which is used to
    pub pair_code_id: u64,
    pub token_code_id: u64,
    pub owner: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// UpdateConfig update relevant code IDs
    UpdateConfig {
        owner: Option<Addr>,
        token_code_id: Option<u64>,
        pair_code_id: Option<u64>,
    },
    /// CreatePair instantiates pair contract
    CreatePair {
        /// Asset infos
        asset_infos: [WeightedAssetInfo; 2],
        /// LBP start time
        start_time: u64,
        /// LBP end time
        end_time: u64,
        /// Pair description
        description: Option<String>,
    },
    Unregister {
        asset_infos: [AssetInfo; 2],
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Config {},
    Pair {
        asset_infos: [AssetInfo; 2],
    },
    Pairs {
        start_after: Option<[AssetInfo; 2]>,
        limit: Option<u32>,
    },
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    pub owner: Addr,
    pub pair_code_id: u64,
    pub token_code_id: u64,
}

/// We currently take no arguments for migrations
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PairsResponse {
    pub pairs: Vec<PairInfo>,
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct FactoryPairInfo {
    pub owner: Addr,
    pub contract_addr: Addr,
}
