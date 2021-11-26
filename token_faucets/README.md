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
near dev-deploy --wasmFile res/greeter.wasm
export ID=dev-1637748940104-87011920759490
```

### Init contract, add oct token as a default token support
```sh
near call $ID new '{}' --accountId $ID
near call $ID default_tokens_support '{}' --accountId $ID
```

Any user can use request_faucet with token id to request a faucet

Request oct token => expected success
```sh
near call $ID request_faucet '{"token_id": "oct.beta_oct_relay.testnet"}' --accountId $ID
```

Request oct token => expected deny (because we already get 1 faucet)
```sh
near call $ID request_faucet '{"token_id": "oct.beta_oct_relay.testnet"}' --accountId $ID
```
