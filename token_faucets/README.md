# Token Faucet Dapp
### Support all kind of NEP141 standard

## Features: 
- One address can receive faucet once time in a day.
- Support many tokens type build on NEP141 standard.

- Only owner can add new faucet token.
- Check balance of token in the wallet.

### Build and deploy 
```sh
cargo build --all --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/*.wasm ./res/
near dev-deploy --wasmFile res/faucet.wasm --accountId faucet_account.testnet
export ID=dev-1637748940104-87011920759490
```

### Init contract, add oct token as a default token support
```sh
near call $ID new '{}' --accountId faucet_account.testnet
near call $ID default_tokens_support '{}' --accountId faucet_account.testnet
```

### Check records map
```sh
near call $ID get_received_faucet '{}' --accountId receive_faucet.testnet
```

Any user can use request_faucet with token id to request a faucet

Send OCT token to the deployed contract on Wallet UI 

Request oct token => expected success
```sh
near call $ID request_faucet '{"token_id": "oct.beta_oct_relay.testnet"}' --accountId receive_faucet.testnet
```

Request oct token => expected deny (because we already get 1 faucet)
```sh
near call $ID request_faucet '{"token_id": "oct.beta_oct_relay.testnet"}' --accountId $ID
```
