use cosmwasm_std::{Addr, Coin, CosmosMsg, Empty, Timestamp};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::state::{AccountData, ScoreData};

/// This needs no info. Owner of the contract is whoever signed the InstantiateMsg.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// Changes the admin
    UpdateAdmin {
        admin: String,
    },
    CreatePair {
        address: String,
        score: String
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // Returns current admin
    Admin {},
    // Get score for one wallet address
    Score { address: String },
    // Shows all scores
    ListScores {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AdminResponse {
    pub admin: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ListScoresResponse {
    pub scores: Vec<ScoreInfo>,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ScoreInfo {
    pub wallet_addr: String,
    pub score: String,
}

impl ScoreInfo {
    pub fn convert(input: ScoreData) -> Self {
        ScoreInfo {
            wallet_addr: input.wallet_addr,
            score: input.score,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ScoreResponse {
    pub wallet_addr: String,
    pub score: String,
}

impl From<ScoreData> for ScoreResponse {
    fn from(input: ScoreData) -> Self {
        ScoreResponse {
            wallet_addr: input.wallet_addr,
            score: input.score,
        }
    }
}