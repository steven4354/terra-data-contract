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

(async () => {
  try {
    const mk = new MnemonicKey({
      mnemonic: process.env.WALLET_MNEMONIC,
    });

    console.log("chainId: \n", process.env.CHAIN_ID);
    console.log("url: \n", process.env.NODE_URL);

    const terra = new LCDClient({
      chainID: String(process.env.CHAIN_ID),
      URL: String(process.env.NODE_URL),
    });

    const wallet = terra.wallet(mk);

    const acctNumber = await wallet.accountNumber()
    console.log(acctNumber);
    
  } catch (e) {
    console.log("Error:\n ", e);
  }
})();
