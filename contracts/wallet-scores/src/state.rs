use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Coin, Storage, Timestamp};
use cosmwasm_storage::{
    bucket, bucket_read, singleton, singleton_read, Bucket, ReadonlyBucket, ReadonlySingleton,
    Singleton,
};
use cw_storage_plus::{Item};

pub const KEY_CONFIG: &[u8] = b"config";
pub const PREFIX_ACCOUNTS: &[u8] = b"accounts";
pub const PREFIX_SCORES: &[u8] = b"accounts";

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
// pub struct PairInfo {
//     pub score: u64,
//     pub address: Addr,
// }

// // u64 -> sequential id
// pub const PAIR_INFO: Map<u64, PairInfo> = Map::new("pair_info");

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// pub struct PairInfo {
//     pub wallet_addr: Addr,
//     pub score: u128,
// }

// pub const PAIR_INFO: Item<PairInfo> = Item::new("pair_info");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Config {
    pub admin: Addr,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
pub struct AccountData {
    /// last block balance was updated (0 is never)
    pub last_update_time: Timestamp,
    /// In normal cases, it should be set, but there is a delay between binding
    /// the channel and making a query and in that time it is empty.
    ///
    /// Since we do not have a way to validate the remote address format, this
    /// must not be of type `Addr`.
    pub remote_addr: Option<String>,
    pub remote_balance: Vec<Coin>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
pub struct ScoreData {
    pub wallet_addr: String,
    pub score: String,
}

pub fn scores(storage: &mut dyn Storage) -> Bucket<ScoreData> {
    bucket(storage, PREFIX_SCORES)
}

pub fn scores_read(storage: &dyn Storage) -> ReadonlyBucket<ScoreData> {
    bucket_read(storage, PREFIX_SCORES)
}

/// accounts is lookup of channel_id to reflect contract
pub fn accounts(storage: &mut dyn Storage) -> Bucket<AccountData> {
    bucket(storage, PREFIX_ACCOUNTS)
}

pub fn accounts_read(storage: &dyn Storage) -> ReadonlyBucket<AccountData> {
    bucket_read(storage, PREFIX_ACCOUNTS)
}

pub fn config(storage: &mut dyn Storage) -> Singleton<Config> {
    singleton(storage, KEY_CONFIG)
}

pub fn config_read(storage: &dyn Storage) -> ReadonlySingleton<Config> {
    singleton_read(storage, KEY_CONFIG)
}
