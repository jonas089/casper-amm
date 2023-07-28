mod utils;
use base64::{engine::general_purpose::STANDARD, Engine};
use casper_engine_test_support::{
    ExecuteRequestBuilder, InMemoryWasmTestBuilder, DEFAULT_ACCOUNT_ADDR,
    PRODUCTION_RUN_GENESIS_REQUEST,
};
use casper_types::bytesrepr::ToBytes;
use casper_types::{
    account::AccountHash, contracts::NamedKeys, runtime_args, ContractHash, Key, RuntimeArgs, URef,
    U256,
};
use utils::create_funded_dummy_account;

pub const ACCOUNT_USER_1: [u8; 32] = [1u8; 32];
pub const ACCOUNT_USER_2: [u8; 32] = [2u8; 32];

#[derive(Clone, Copy)]
pub struct Sender(pub AccountHash);

#[cfg(test)]
pub struct TestContext {
    pub builder: InMemoryWasmTestBuilder,
    pub ali: AccountHash,
    pub bob: AccountHash,
}

impl TestContext {
    pub fn new() -> TestContext {
        let mut builder = InMemoryWasmTestBuilder::default();
        builder.run_genesis(&PRODUCTION_RUN_GENESIS_REQUEST);
        let ali = create_funded_dummy_account(&mut builder, Some(ACCOUNT_USER_1));
        let bob = create_funded_dummy_account(&mut builder, Some(ACCOUNT_USER_2));

        TestContext { builder, ali, bob }
    }

    pub fn named_keys(&self) -> NamedKeys {
        self.builder
            .get_expected_account(*DEFAULT_ACCOUNT_ADDR)
            .named_keys()
            .clone()
    }

    pub fn contract_named_keys(&self, contract_name: &str, key_name: &str) -> Key {
        let contract_hash = self.contract_hash_from_named_keys(contract_name);
        *self
            .builder
            .get_contract(contract_hash)
            .expect("should have contract")
            .named_keys()
            .get(key_name)
            .unwrap()
    }

    pub fn contract_hash_from_named_keys(&self, key_name: &str) -> ContractHash {
        self.named_keys()
            .get(key_name)
            .expect("must have contract hash key as part of contract creation")
            .into_hash()
            .map(ContractHash::new)
            .expect("must get contract hash")
    }

    // pub fn seed_uref_contract(&self, name: &str, contract_name: &str) -> URef {
    //     let contract_key: Key = self.contract_hash_from_named_keys(contract_name).into();
    //     *self
    //         .builder
    //         .query(None, contract_key, &[])
    //         .expect("contract exists")
    //         .as_contract()
    //         .expect("convert contract")
    //         .named_keys()
    //         .get(name)
    //         .expect("must have key")
    //         .as_uref()
    //         .expect("must convert to seed uref")
    // }

    pub fn install(&mut self) {
        let session_args_a = runtime_args! {
            // initialise cep18
            "name" => "TOKEN_A".to_string(),
            "symbol" => "ATKN".to_string(),
            "decimals" => 18_u8,
            // 1000_000 Tokens (1 * 10 ** 24 WEI)
            "total_supply" => U256::from(1_000_000_000_000_000_000_000_000_u128),
        };
        let session_args_b = runtime_args! {
            // initialise cep18
            "name" => "TOKEN_B".to_string(),
            "symbol" => "BTKN".to_string(),
            "decimals" => 18_u8,
            "total_supply" => U256::from(1_000_000_000_000_000_000_000_000_u128),
        };
        let session_args_c = runtime_args! {
            // initialise cep18
            "name" => "TOKEN_C".to_string(),
            "symbol" => "CTKN".to_string(),
            "decimals" => 18_u8,
            "total_supply" => U256::from(1_000_000_000_000_000_000_000_000_u128),
        };
        let a_exec_request: casper_execution_engine::core::engine_state::ExecuteRequest =
            ExecuteRequestBuilder::standard(
                *DEFAULT_ACCOUNT_ADDR,
                "../wasm/cep18.wasm",
                session_args_a,
            )
            .build();
        let b_exec_request: casper_execution_engine::core::engine_state::ExecuteRequest =
            ExecuteRequestBuilder::standard(
                *DEFAULT_ACCOUNT_ADDR,
                "../wasm/cep18.wasm",
                session_args_b,
            )
            .build();
        let c_exec_request: casper_execution_engine::core::engine_state::ExecuteRequest =
            ExecuteRequestBuilder::standard(
                *DEFAULT_ACCOUNT_ADDR,
                "../wasm/cep18.wasm",
                session_args_c,
            )
            .build();
        self.builder.exec(a_exec_request).expect_success().commit();
        self.builder.exec(b_exec_request).expect_success().commit();
        self.builder.exec(c_exec_request).expect_success().commit();

        let a_contract_hash = self.contract_hash_from_named_keys("cep18_contract_hash_TOKEN_A");
        let b_contract_hash = self.contract_hash_from_named_keys("cep18_contract_hash_TOKEN_B");
        let c_contract_hash = self.contract_hash_from_named_keys("cep18_contract_hash_TOKEN_C");

        let session_args = runtime_args! {
            "token0" => Key::from(a_contract_hash),
            "token1" => Key::from(b_contract_hash),
            "token" => Key::from(c_contract_hash)
        };
        let contract_exec_request: casper_execution_engine::core::engine_state::ExecuteRequest =
            ExecuteRequestBuilder::standard(
                *DEFAULT_ACCOUNT_ADDR,
                "../wasm/contract.wasm",
                session_args,
            )
            .build();
        self.builder
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
            *DEFAULT_ACCOUNT_ADDR,
            contract_hash,
            "mint",
            session_args
        ).build();

        *self.builder
            .exec(mint_request)
            .commit();
    }*/

    // call transfer directly on cep18
    pub fn transfer(
        &mut self,
        msg_sender: AccountHash,
        recipient: Key,
        amount: U256,
        contract_hash: ContractHash,
    ) {
        let session_args = runtime_args! {
            "recipient" => recipient,
            "amount" => amount
        };

        let transfer_request = ExecuteRequestBuilder::contract_call_by_hash(
            msg_sender,
            contract_hash,
            "transfer",
            session_args,
        )
        .build();

        self.builder
            .exec(transfer_request)
            .expect_success()
            .commit();
    }

    // call transfer through contract
    pub fn cross_contract_transfer(
        &mut self,
        msg_sender: AccountHash,
        recipient: Key,
        amount: U256,
        cep18_contract_hash: ContractHash,
    ) {
        let session_args = runtime_args! {
            "recipient" => recipient,
            "amount" => amount,
            // the cep 18 contract
            "contract_hash" => cep18_contract_hash
        };

        let tst = self.contract_hash_from_named_keys("casper_automated_market_maker");
        dbg!(self.named_keys());
        dbg!(tst);

        let transfer_request = ExecuteRequestBuilder::contract_call_by_hash(
            msg_sender,
            // the contract that's supposed to call the cep18
            self.contract_hash_from_named_keys("casper_automated_market_maker"),
            "test_transfer",
            session_args,
        )
        .build();

        self.builder
            .exec(transfer_request)
            .expect_success()
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

        *self.builder
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

        *self.builder
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

        *self.builder
            .exec(swap_request)
            .commit();
    }
    */

    pub fn balance_of(&self, account: Key, contract_name: &str) -> U256 {
        let seed_uref: URef = *self
            .contract_named_keys(contract_name, "balances")
            .as_uref()
            .unwrap();
        let dictionary_key = make_dictionary_item_key(account);
        self.builder
            .query_dictionary_item(None, seed_uref, &dictionary_key)
            .unwrap()
            .as_cl_value()
            .unwrap()
            .clone()
            .into_t()
            .unwrap()
    }
}
impl Default for TestContext {
    fn default() -> Self {
        Self::new()
    }
}

fn make_dictionary_item_key(owner: Key) -> String {
    let preimage = owner.to_bytes().unwrap();
    STANDARD.encode(preimage)
}
