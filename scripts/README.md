# Deploying a contracts to the Terra Station. #

## Bootstrap verifier
* This demo uses `package.json` to bootstrap all dependencies.
  ```shell
  $ cp sample.local.env .env
  $ npm install
  $ npm start
  ```

### Overview a scripts
* This script deploys all contracts to testnet
  ```shell
  npm run build-app
  ```

### Output Result
* As a result, we will get a data file `<chain-id>.json` located in the root folder by default.
  
```json
  {
  "astroport_lbp_token": {
    "ID": 30101,
    "Addr": "terra1aj9stfkw6qea2nf9zzrdzchylq5ayjhz7f503r"
  },
  "wallet_scores": {
    "ID": 30182,
    "Addr": "terra1c02xnzgme3502tq8fs4er3q8enk6pzh8hdh7zd"
  }
}```
