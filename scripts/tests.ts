import 'dotenv/config'
import { executeContract, newClient, queryContract, readNetworkConfig } from './helpers.js';

// a different address to test changing ownership
const NEW_ADMIN = "terra1qg39df9w7wl5rd70spqdn3xvqj4cfe3t8vsldn";
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

    // query get owner
    const res2 = await queryContract(
      client.terra, 
      networkConfig.ibc_reflect_send.Addr,
      {
        admin: {}
      }
    )
    console.log(res2);

    // set scores
    const res3 = await executeContract(
      client.terra, 
      client.wallet, 
      networkConfig.ibc_reflect_send.Addr,
      {
        create_pair: {
          address: "address_2",
          score: "20"
        }
      }
    )
    DEBUG && console.log(res3);

    const res4 = await executeContract(
      client.terra, 
      client.wallet, 
      networkConfig.ibc_reflect_send.Addr,
      {
        create_pair: {
          address: "address_3",
          score: "20"
        }
      }
    )
    DEBUG && console.log(res4);

    // query scores
    const res5 = await queryContract(
      client.terra, 
      networkConfig.ibc_reflect_send.Addr,
      {
        list_scores: {}
      }
    )
    console.log(res5);

    // after changing admin should not be able to set score (update DEBUG to true)s
    const res6 = DEBUG && await executeContract(
        client.terra,
        client.wallet,
        networkConfig.ibc_reflect_send.Addr,
        {
            update_admin: {
                admin: NEW_ADMIN
            },
        }
    )
    DEBUG && console.log(res6);

    // trying to set scores after admin change should fail (update DEBUG to true)
    const res7 = DEBUG && await executeContract(
      client.terra,
      client.wallet,
      networkConfig.ibc_reflect_send.Addr,
      {
        create_pair: {
          address: "address_3",
          score: "20"
        }
      }
    )
    DEBUG && console.log(res7);

    // querying a single score from a wallet address
    const res8 = await queryContract(
      client.terra,
      networkConfig.ibc_reflect_send.Addr,
      {
        score: {
          address: "address_2"
        }
      }
    )
    console.log(res8);

  } catch (e) {
    console.log("Error:\n ", e);
  }
})();
