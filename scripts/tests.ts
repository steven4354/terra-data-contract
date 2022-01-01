import 'dotenv/config'
import { executeContract, newClient, queryContract, readNetworkConfig } from './helpers.js';

// a different address to test changing ownership
const NEW_ADMIN = "terra1qg39df9w7wl5rd70spqdn3xvqj4cfe3t8vsldn";
const DEBUG = true;

(async () => {
  try {
    const client = newClient();    
    const networkConfig = readNetworkConfig(client.terra.config.chainID);

    // TEST: query get owner
    const res2 = await queryContract(
      client.terra, 
      networkConfig.wallet_scores.Addr,
      {
        admin: {}
      }
    )
    console.log(res2);

    // TEST: set scores
    const res3 = await executeContract(
      client.terra, 
      client.wallet, 
      networkConfig.wallet_scores.Addr,
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
      networkConfig.wallet_scores.Addr,
      {
        create_pair: {
          address: "address_3",
          score: "20"
        }
      }
    )
    DEBUG && console.log(res4);

    // TEST: query scores
    const res5 = await queryContract(
      client.terra, 
      networkConfig.wallet_scores.Addr,
      {
        list_scores: {}
      }
    )
    console.log(res5);

    // TEST: after changing admin should not be able to set score (update DEBUG to true)s
    const res6 = DEBUG && await executeContract(
        client.terra,
        client.wallet,
        networkConfig.wallet_scores.Addr,
        {
            update_admin: {
                admin: NEW_ADMIN
            },
        }
    )
    DEBUG && console.log(res6);

    // TEST: trying to set scores after admin change should fail (update DEBUG to true)
    const res7 = DEBUG && await executeContract(
      client.terra,
      client.wallet,
      networkConfig.wallet_scores.Addr,
      {
        create_pair: {
          address: "address_3",
          score: "20"
        }
      }
    )
    DEBUG && console.log(res7);

    // TEST: querying a single score from a wallet address
    const res8 = await queryContract(
      client.terra,
      networkConfig.wallet_scores.Addr,
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
