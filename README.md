# terra-data-contract

simple terra/cosm wasm contract to set/retrieve data using Buckets & allowing only admin access to certain functionality (setting/changing data)

contract is deployed to bombay-12 here: https://finder.terra.money/testnet/address/terra1nk2egwl93j4f022flleyvw7xkysz6kqxh9yntf

demo of contract: https://youtu.be/YzAVqHMdb-Y

## getting started

each contract can be individually compiled within their folders in `/contracts` details are in the README.mds in those folders

after compilation, move the .wasm file to `/artifacts` and run `npm install; npm start` within `/scripts` to deloy all the contracts

to see the contracts in action, afterwards, run `npm run test` inside `/scripts`

## inspired by the below contracts

https://github.com/steven4354/terracamp-delta0x/blob/main/deploy/scripts/delta0x_test.ts

https://github.com/steven4354/astroport-lbport

https://github.com/steven4354/cw-plus

https://github.com/steven4354/cosmwasm-0.16.0/tree/main/contracts/ibc-reflect-send

