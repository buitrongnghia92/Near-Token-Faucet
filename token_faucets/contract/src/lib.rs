use near_sdk::borsh::{
    self, 
    BorshDeserialize, 
    BorshSerialize
};

use near_sdk::collections::UnorderedMap;
use near_sdk::serde::{
    Deserialize, 
    Serialize
};

use std::convert::TryFrom;
use near_sdk::serde_json::json;
use near_sdk::json_types::{
    U128, 
    WrappedBalance, 
    WrappedDuration, 
    ValidAccountId
};

use near_sdk::{
    env, 
    near_bindgen, 
    AccountId, 
    Balance, 
    setup_alloc, 
    Promise, 
    Timestamp, 
    Gas,
};

const DEFAULT_GAS_TO_PAY: Gas = 20_000_000_000_000;
const A_DAY_UNIX_TS: u64 = 86400;

setup_alloc!();

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize)]
pub struct FaucetContract {
    owner: ValidAccountId,
    supported_tokens_map: UnorderedMap<ValidAccountId, Balance>,
    users_record_map: UnorderedMap<ValidAccountId, Timestamp> 
}

impl Default for FaucetContract {
    fn default() -> Self {
        env::panic(b"Faucet should be initialized before usage")
    }
}

#[near_bindgen]
impl FaucetContract {
    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "Already initialized");

        let _owner = ValidAccountId::try_from(env::signer_account_id().clone()).unwrap();

        let this = Self {
            owner: _owner.clone(),
            supported_tokens_map: UnorderedMap::new(b"a".to_vec()),
            users_record_map: UnorderedMap::new(b"u".to_vec())
        };

        return this;
    }
    
    pub fn default_tokens_support(&mut self) {
        let _oct_token = ValidAccountId::try_from("oct.beta_oct_relay.testnet").unwrap();
        let _oct_amount = 1 as u128;
        self.supported_tokens_map.insert(&_oct_token, &_oct_amount);
    }

    pub fn request_faucet_token(&mut self, token_id: ValidAccountId, faucet_amount: Balance) {
        self.only_owner();
        self.supported_tokens_map.insert(&token_id, &faucet_amount);
    }

    
    // #[payable]
    pub fn request_faucet(&mut self, token_id: AccountId) -> bool {
        let _signer = env::signer_account_id();
        let _valid_signer_id = ValidAccountId::try_from(_signer.clone()).unwrap();

        assert!(
            !self.is_allow_faucet(_valid_signer_id), 
            "You already get faucet today!"
        );

        let _valid_token_id = ValidAccountId::try_from(token_id.clone()).unwrap();

        match self.supported_tokens_map.get(&_valid_token_id) {
            Some(value) => {

               Promise::new(token_id.clone()).function_call(
                    b"ft_transfer".to_vec(),
                    json!({
                        "receiver_id": env::signer_account_id(),
                        "amount": WrappedBalance::from(value),
                    }).to_string().as_bytes().to_vec(), 
                    1, DEFAULT_GAS_TO_PAY,
                );

               self.record_faucet(_valid_token_id);
               return true;
            },
            None =>  {return false;}
        }
    }

    pub fn get_tokens_support(&mut self) -> Vec<(ValidAccountId, Balance)> {
        self.supported_tokens_map
            .iter()
            .collect()
    }

    pub fn get_received_faucet(&mut self) -> Vec<(ValidAccountId, Timestamp)> {
        self.users_record_map
            .iter()
            .collect()
    }

    fn record_faucet(&mut self, faucet_account: ValidAccountId) {
        self.users_record_map.insert(&faucet_account, &env::block_timestamp());
    }

    fn is_allow_faucet(&self, faucet_account: ValidAccountId) -> bool {
        match self.users_record_map.get(&faucet_account) {
            Some(value) => {
                env::block_timestamp() - value >= A_DAY_UNIX_TS
            },
            None => false 
        }
    }

     fn only_owner(&self) {
        let predecessor = env::predecessor_account_id();
        let receiver_id = ValidAccountId::try_from(predecessor.clone()).unwrap();

        assert_eq!(
            &receiver_id,
            &self.owner,
            "Only contract owner can call this fn"
            );
    }
}

