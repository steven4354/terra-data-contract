import 'dotenv/config'
import {
  LCDClient,
  MsgStoreCode,
  MnemonicKey,
  isTxError,
  MsgExecuteContract,
  MsgInstantiateContract,
} from "@terra-money/terra.js";
import * as fs from "fs";
import { executeContract, newClient, queryContract, readNetworkConfig } from './helpers.js';
import { configDefault } from './deploy_configs';

// new admin
const NEW_ADMIN = "terra1w25svpvjvgl5akzkgsuxwrhgsazxrr5gcltaks";
const DEBUG = false;

(async () => {
  try {
    const client = newClient();    
    const networkConfig = readNetworkConfig(client.terra.config.chainID);

    /*
    'rpc error: code = InvalidArgument desc = failed to execute message; message index: 0: 
    Error parsing into type ibc_reflect_send::msg::ExecuteMsg: unknown variant `deposit`, 
    expected one of `update_admin`, `send_msgs`, `check_remote_balance`, `send_funds`: 
    execute wasm contract failed: invalid request'

    rpc error: code = InvalidArgument desc = failed to 
    execute message; message index: 0: Error parsing into type ibc_reflect_send::msg::ExecuteMsg: 
    missing field `admin`: execute wasm contract failed: invalid request
    */

    // test setting new admin
    const res = await executeContract(
      client.terra, 
      client.wallet, 
      networkConfig.ibc_reflect_send.Addr,
      {
        update_admin: {
          admin: NEW_ADMIN
        },
      }
    )
    DEBUG && console.log(res);

    // query accounts
    const res2 = await await queryContract(
      client.terra, 
      networkConfig.ibc_reflect_send.Addr,
      {
        list_accounts: {}
      }
    )
    console.log(res2);

    // execute get owner
    const res3 = await executeContract(
      client.terra, 
      client.wallet, 
      networkConfig.ibc_reflect_send.Addr,
      {
        get_admin: {}
      }
    )
    DEBUG && console.log(res3);

    // query get owner
    const res4 = await queryContract(
      client.terra, 
      networkConfig.ibc_reflect_send.Addr,
      {
        admin: {}
      }
    )
    console.log(res4);

  } catch (e) {
    console.log("Error:\n ", e);
  }
})();
