use cosmwasm_std::{
    entry_point, to_binary, CosmosMsg, Deps, DepsMut, Env, IbcMsg, MessageInfo, Order,
    QueryResponse, Response, StdError, StdResult, Addr
};

use crate::ibc::PACKET_LIFETIME;
use crate::ibc_msg::PacketMsg;
use crate::msg::{AdminResponse, ExecuteMsg, InstantiateMsg, QueryMsg, ListScoresResponse, ScoreInfo, ScoreResponse};
use crate::state::{accounts, accounts_read, config, config_read, Config, scores, scores_read, ScoreData};
use cosmwasm_storage::Bucket;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    // store who the admin is
    let cfg = Config { admin: info.sender };
    config(deps.storage).save(&cfg)?;

    Ok(Response::new().add_attribute("action", "instantiate"))
}

#[entry_point]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        ExecuteMsg::UpdateAdmin { admin } => handle_update_admin(deps, info, admin),
        ExecuteMsg::CreatePair { address, score } => handle_store_address_score(deps, info, address, score),
    }
}

pub fn handle_update_admin(
    deps: DepsMut,
    info: MessageInfo,
    new_admin: String,
) -> StdResult<Response> {
    // auth check
    let mut cfg = config(deps.storage).load()?;
    if info.sender != cfg.admin {
        return Err(StdError::generic_err("Only admin may set new admin"));
    }
    cfg.admin = deps.api.addr_validate(&new_admin)?;
    config(deps.storage).save(&cfg)?;

    Ok(Response::new()
        .add_attribute("action", "handle_update_admin")
        .add_attribute("new_admin", cfg.admin))
}

pub fn handle_store_address_score(
    deps: DepsMut,
    info: MessageInfo,
    address: String,
    score: String
) -> StdResult<Response> {
    // auth check
    let cfg = config(deps.storage).load()?;
    if info.sender != cfg.admin {
        return Err(StdError::generic_err("Only admin may update scores"));
    }

    // TODO: below is a better implementation with Map, if have time update contract version (to 0.16.3) to support this
    // future_reference: requires new version of cosm wasm to use the Maps type, i.e. astroport supports it
    // 0.16.1 to 0.16.3 is prob the write ones
    // let mut config: PairInfo = PAIR_INFO.load(deps.storage)?;
    // // PAIRS.save(
    // //     deps.storage,
    // //     0,
    // //     &PairInfo {
    // //         score: score,
    // //         address: address,
    // //     }
    // // )?;
    // let pair_info: &PairInfo = &PairInfo {
    //     wallet_addr: address,
    //     score: score,
    // };
    // PAIR_INFO.save(deps.storage, pair_info)?;
    // let scores = scores(deps.storage).load()?;

    // @dev future_reference: saves multiple but stores empty address and scores
    // due to not updating the store
    // let wallet_raw = address.as_bytes();
    // let mut scores = scores(deps.storage);
    // scores.update(&wallet_raw, |score| -> StdResult<_> {
    //     // score.state.address = address;
    //     // score.state.score = score;
    //     Ok(score.unwrap_or_default())
    // })?;
    
    let address_copy = address.clone();
    let wallet_raw = address.as_bytes();
    let score = ScoreData { wallet_addr: address_copy, score };

    let mut scores: Bucket<ScoreData> = scores(deps.storage);
    scores.save(
        &wallet_raw,
        &score
    );

    Ok(Response::new()
        .add_attribute("action", "handle_store_address_score"))
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<QueryResponse> {
    match msg {
        QueryMsg::Admin {} => to_binary(&query_admin(deps)?),
        QueryMsg::Score  { address } => to_binary(&query_score(deps, address)?),
        QueryMsg::ListScores {} => to_binary(&query_list_scores(deps)?)
    }
}

fn query_score(deps: Deps, address: String) -> StdResult<ScoreResponse> {
    let score = scores_read(deps.storage).load(address.as_bytes())?;
    Ok(score.into())
}

fn query_list_scores(deps: Deps) -> StdResult<ListScoresResponse> {
    let scores: StdResult<Vec<_>> = scores_read(deps.storage)
        .range(None, None, Order::Ascending)
        .map(|r| {
            let (k, score) = r?;
            Ok(ScoreInfo::convert(score))
        })
        .collect();
    Ok(ListScoresResponse {
        scores: scores?,
    })
}

fn query_admin(deps: Deps) -> StdResult<AdminResponse> {
    let Config { admin } = config_read(deps.storage).load()?;
    Ok(AdminResponse {
        admin: admin.into(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

    const CREATOR: &str = "creator";

    #[test]
    fn instantiate_works() {
        let mut deps = mock_dependencies(&[]);
        let msg = InstantiateMsg {};
        let info = mock_info(CREATOR, &[]);
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        let admin = query_admin(deps.as_ref()).unwrap();
        assert_eq!(CREATOR, admin.admin.as_str());
    }

    // #[test]
    // needs better logging to see errors
    // fn handle_store_address_score_works() {
    //     let mut deps = mock_dependencies(&[]);

    //     // instantiate contract
    //     let msg = InstantiateMsg {};
    //     let info1 = mock_info(CREATOR, &[]);
    //     instantiate(deps.as_mut(), mock_env(), info1, msg).unwrap();

    //     // set new wallet & wallet score
    //     let info2 = mock_info(CREATOR, &[]);
    //     let test_addr: String = "address_1".to_string();
    //     let test_score: String = "10".to_string();
    //     let res2 = handle_store_address_score(deps.as_mut(), info2, test_addr, test_score).unwrap();
    //     assert_eq!(res2.messages.len(), res2.messages.len());

    //     let scores_res = query_list_scores(deps.as_ref()).unwrap();
    //     assert_eq!(0, scores_res.scores.len());
    //     // assert_eq!(CREATOR, admin.admin.as_str());
    // }
}
