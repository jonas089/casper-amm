use casper_engine_test_support::{
    ExecuteRequestBuilder, InMemoryWasmTestBuilder, DEFAULT_ACCOUNT_INITIAL_BALANCE,
    DEFAULT_GENESIS_CONFIG, DEFAULT_GENESIS_CONFIG_HASH, WasmTestBuilder,
};
use casper_execution_engine::{core::engine_state::{
    run_genesis_request::RunGenesisRequest, GenesisAccount,
}, storage::global_state::in_memory::InMemoryGlobalState};
use casper_types::{
    account::{AccountHash, Account},
    bytesrepr::{FromBytes, ToBytes},
    runtime_args, URef, AsymmetricType, CLTyped, ContractHash, Key, PublicKey, RuntimeArgs, U256, U512, Motes, SecretKey, contracts::{NamedKeys}
};
use casper_contract::{unwrap_or_revert::UnwrapOrRevert};

#[derive(Clone, Copy)]
pub struct Sender(pub AccountHash);

pub struct TestEnv {
    context: WasmTestBuilder<InMemoryGlobalState>,
    pub zero: AccountHash,
    pub ali: AccountHash,
    pub bob: AccountHash
}

pub fn make_dictionary_item_key(owner: Key) -> String {
    let preimage = owner.to_bytes().unwrap();
    base64::encode(preimage)
}

impl TestEnv {
    pub fn new() -> TestEnv {
        // Token zero address
        let secret_key_zero = SecretKey::ed25519_from_bytes([9u8; 32]).unwrap();
        let public_key_zero = PublicKey::from(&secret_key_zero);
        let zero = AccountHash::from(&public_key_zero);
        let zero_account = GenesisAccount::account(
            public_key_zero,
            Motes::new(U512::from(DEFAULT_ACCOUNT_INITIAL_BALANCE)),
            None,
        );
        // Account "ali"
        let secret_key_ali = SecretKey::ed25519_from_bytes([7u8; 32]).unwrap();
        let public_key_ali = PublicKey::from(&secret_key_ali);
        let ali = AccountHash::from(&public_key_ali);
        let ali_account = GenesisAccount::account(
            public_key_ali,
            Motes::new(U512::from(DEFAULT_ACCOUNT_INITIAL_BALANCE)),
            None,
        );
        // Account "bob"
        let secret_key_bob = SecretKey::ed25519_from_bytes([8u8; 32]).unwrap();
        let public_key_bob = PublicKey::from(&secret_key_bob);
        let bob = AccountHash::from(&public_key_bob);
        let bob_account = GenesisAccount::account(
            public_key_bob,
            Motes::new(U512::from(DEFAULT_ACCOUNT_INITIAL_BALANCE)),
            None,
        );
        let mut genesis_config = DEFAULT_GENESIS_CONFIG.clone();
        genesis_config.ee_config_mut().push_account(ali_account);
        genesis_config.ee_config_mut().push_account(bob_account);
        genesis_config.ee_config_mut().push_account(zero_account);
        let run_genesis_request = RunGenesisRequest::new(
            *DEFAULT_GENESIS_CONFIG_HASH,
            genesis_config.protocol_version(),
            genesis_config.take_ee_config(),
        );
        let mut context: WasmTestBuilder<InMemoryGlobalState> = InMemoryWasmTestBuilder::default();      
        context.run_genesis(&run_genesis_request).commit();
        TestEnv {
            context,
            zero,
            bob,
            ali
        }
    }

    pub fn default_account(&self) -> AccountHash {
        self.ali
    }

    pub fn named_keys(&self) -> NamedKeys {
        self.context
            .get_expected_account(self.zero)
            .named_keys()
            .clone()
    }

    pub fn contract_hash(&self, name: &str) -> ContractHash{
        self.context
            .get_expected_account(self.zero)
            .named_keys()
            .get(name)
            .expect("must have contract hash key as part of contract creation")
            .into_hash()
            .map(ContractHash::new)
            .expect("must get contract hash")
    }

    pub fn contract_key(&self, name: &str) -> Key{
        let contract_hash = self.contract_hash(name);
        Key::from(contract_hash)
    }

    pub fn seed_uref_default_account(&self, name: &str) -> URef{
        let default_account_key: Key = Key::from(self.zero);
        *self.context
            .query(None, default_account_key, &[])
            .expect("contract exists")
            .as_account()
            .expect("convert contract")
            .named_keys()
            .get(name)
            .expect("must have key")
            .as_uref()
            .expect("must convert to seed uref")
    }

    pub fn seed_uref_contract(&self, name: &str, contract_name: &str) -> URef {
        let contract_key: Key = self.contract_key(contract_name);
        *self.context
            .query(None, contract_key, &[])
            .expect("contract exists")
            .as_contract()
            .expect("convert contract")
            .named_keys()
            .get(name)
            .expect("must have key")
            .as_uref()
            .expect("must convert to seed uref")
    }

    pub fn install(&mut self){
        let session_args_a = runtime_args! {
            // initialise cep18
            "name" => "TOKEN_A".to_string(),
            "symbol" => "ATKN".to_string(),
            "decimals" => u8::from(18),
            // 1000_000 Tokens (1 * 10 ** 24 WEI)
            "total_supply" => U256::from(1000_000_000_000_000_000_000_000u128),
        };
        let session_args_b = runtime_args! {
            // initialise cep18
            "name" => "TOKEN_B".to_string(),
            "symbol" => "BTKN".to_string(),
            "decimals" => u8::from(18),
            "total_supply" => U256::from(1000_000_000_000_000_000_000_000u128),
        };
        let session_args_c = runtime_args! {
            // initialise cep18
            "name" => "TOKEN_C".to_string(),
            "symbol" => "CTKN".to_string(),
            "decimals" => u8::from(18),
            "total_supply" => U256::from(1000_000_000_000_000_000_000_000u128),
        };
        let a_exec_request: casper_execution_engine::core::engine_state::ExecuteRequest =
            ExecuteRequestBuilder::standard(self.zero, "../wasm/cep18.wasm", session_args_a)
            .build();
        let b_exec_request: casper_execution_engine::core::engine_state::ExecuteRequest = 
            ExecuteRequestBuilder::standard(self.zero, "../wasm/cep18.wasm", session_args_b)
            .build();
        let c_exec_request: casper_execution_engine::core::engine_state::ExecuteRequest =
            ExecuteRequestBuilder::standard(self.zero, "../wasm/cep18.wasm", session_args_c)
            .build();
        self.context
            .exec(a_exec_request)
            .expect_success()
            .commit();
        self.context
            .exec(b_exec_request)
            .expect_success()
            .commit();
        self.context
            .exec(c_exec_request)
            .expect_success()
            .commit();

        let a_contract_hash = self.contract_hash("cep18_contract_hash_TOKEN_A");
        let b_contract_hash = self.contract_hash("cep18_contract_hash_TOKEN_B");
        let c_contract_hash = self.contract_hash("cep18_contract_hash_TOKEN_C");

        let session_args = runtime_args! {
            "token0" => a_contract_hash,
            "token1" => b_contract_hash,
            "token" => c_contract_hash
        };
        let contract_exec_request: casper_execution_engine::core::engine_state::ExecuteRequest = 
            ExecuteRequestBuilder::standard(self.zero, "../wasm/contract.wasm", session_args)
            .build();
        self.context
            .exec(contract_exec_request)
            .expect_success()
            .commit();
    }
    /* disabled,- premine only
    pub fn mint(&mut self, owner: Key, amount: U256, contract_hash: ContractHash){
        let session_args = runtime_args!{
            "owner" => owner,
            "amount" => amount
        };
        let mint_request = ExecuteRequestBuilder::contract_call_by_hash(
            self.zero,
            contract_hash,
            "mint",
            session_args
        ).build();

        self.context
            .exec(mint_request)
            .commit();
    }*/
    // call transfer directly on cep18
    pub fn transfer(&mut self, msg_sender: AccountHash, recipient: Key, amount: U256, contract_hash: ContractHash){
        let session_args = runtime_args!{
            "recipient" => recipient,
            "amount" => amount
        };

        let transfer_request = ExecuteRequestBuilder::contract_call_by_hash(
            msg_sender,
            contract_hash,
            "transfer",
            session_args
        ).build();

        self.context
            .exec(transfer_request).expect_success()
            .commit();
    }
    // call transfer through contract
    pub fn cross_contract_transfer(&mut self, msg_sender: AccountHash, recipient: Key, amount: U256, cep18_contract_hash: ContractHash){
        let session_args = runtime_args!{
            "recipient" => recipient,
            "amount" => amount,
            // the cep 18 contract
            "contract_hash" => cep18_contract_hash
        };

        let transfer_request = ExecuteRequestBuilder::contract_call_by_hash(
            msg_sender,
            // the contract that's supposed to call the cep18
            self.contract_hash("casper_automated_market_maker"),
            "test_transfer",
            session_args
        ).build();

        self.context
            .exec(transfer_request).expect_success()
            .commit();
    }
    /* 
    pub fn transfer_from(&mut self, msg_sender: AccountHash, recipient: Key, owner: Key, amount: U256, contract_hash: ContractHash){
        let session_args = runtime_args!{
            "recipient" => recipient,
            "owner" => owner,
            "amount" => amount
        };

        let transfer_request = ExecuteRequestBuilder::contract_call_by_hash(
            msg_sender,
            contract_hash,
            "transfer_from",
            session_args
        ).build();

        self.context
            .exec(transfer_request)
            .commit();
    }

    pub fn approve(&mut self, msg_sender: AccountHash, spender: Key, amount: U256, contract_hash: ContractHash){
        let session_args = runtime_args! {
            "spender" => spender,
            "amount" => amount
        };

        let approve_request = ExecuteRequestBuilder::contract_call_by_hash(
            msg_sender,
            contract_hash,
            "approve",
            session_args
        ).build();

        self.context
            .exec(approve_request)
            .commit();
    }

    pub fn swap(&mut self, msg_sender: AccountHash, from_token: &str, amount: U256){
        let contract_hash = self.contract_hash("casper_automated_market_maker");
        let fromToken = self.contract_hash(from_token);
        let session_args = runtime_args! {
            "fromToken" => fromToken,
            "amount" => amount
        };

        let swap_request = ExecuteRequestBuilder::contract_call_by_hash(
            msg_sender,
            contract_hash,
            "swap",
            session_args
        ).build();

        self.context
            .exec(swap_request)
            .commit();
    }
    */
    pub fn balance_of(&self, account: Key, contract_name: &str) -> U256{
        let seed_uref = self.seed_uref_contract("balances", contract_name);
        let dictionary_key = make_dictionary_item_key(account);
        self.context.query_dictionary_item(None, seed_uref, &dictionary_key)
            .unwrap()
            .as_cl_value()
            .unwrap()
            .clone()
            .into_t()
            .unwrap()
    }
    
}