#[cfg(test)]
mod test_fixture;

#[cfg(test)]
mod tests {
    use crate::test_fixture::{Sender, TestEnv};
    use casper_types::bytesrepr::ToBytes;
    use casper_contract::unwrap_or_revert::UnwrapOrRevert;
    use casper_types::{Key, U256};
    #[test]
    fn should_install() {
        let mut fixture: TestEnv = TestEnv::new();
        fixture.install();
    }

    #[test]
    fn cross_contract(){
        let mut fixture: TestEnv = TestEnv::new();
        fixture.install();
        let contract_hash = fixture.contract_hash("casper_automated_market_maker");
        let token0 = fixture.contract_hash("cep18_contract_hash_TOKEN_A");
        let token1 = fixture.contract_hash("cep18_contract_hash_TOKEN_B");

        // Hash: contract_hash, contract_package, contract_wasm
        let named_keys = fixture.named_keys();
        let contract_package_key = named_keys.get("casper_automated_market_maker").expect("should have package hash");

        fixture.transfer(fixture.zero, contract_package_key.clone(), U256::from(10000), token0, Key::from(fixture.zero));
        let balance_contract: U256 = fixture.balance_of(contract_package_key.clone(), "cep18_contract_hash_TOKEN_A");
        assert_eq!(balance_contract, U256::from(10000));

        // call a contract that calls the token0: ContractHash transfer endpoint
        // this does not seem to work.
        fixture.cross_contract_transfer(fixture.ali, Key::from(fixture.bob), U256::from(5000), token0, Key::from(fixture.zero));
        /* both contract & cep18 use immediate caller
            pub(crate) fn get_immediate_caller_address() -> Result<Key, Error> {
                let call_stack = runtime::get_call_stack();
                call_stack
                    .into_iter()
                    .rev()
                    .nth(1)
                    .map(call_stack_element_to_address)
                    .ok_or(Error::InvalidContext)
            } 
            
            -> cep18 should charge the contract, not ali        
         */
        // check if cep18 charged the contract
        // this is 10000, but it should be 5000...
        //let balance_contract: U256 = fixture.balance_of(Key::from(fixture.ali), "cep18_contract_hash_TOKEN_A");
        //assert_eq!(balance_contract, U256::from(5000));

        // check if the contract charged ali instead
        // same result (10000) 
        //let balance_ali: U256 = fixture.balance_of(Key::from(fixture.ali), "cep18_contract_hash_TOKEN_A");
        //assert_eq!(balance_ali, U256::from(5000));



        //let balance_bob: U256 = fixture.balance_of(Key::from(fixture.bob), "cep18_contract_hash_TOKEN_A");
        //assert_eq!(balance_bob, U256::from(5000));
    }
}
/* 
    #[test]
    fn swap(){
        // first fund the AMM contract with token A and B.
        // then fund ali with token A
        // then approve the contract to spend funds in the name of ali
        // then call the swap
        // cep18_contract_hash_TOKEN_[ID]
        // casper_automated_market_maker
        let mut fixture: TestEnv = TestEnv::new();
        fixture.install();
        let contract_hash = fixture.contract_hash("casper_automated_market_maker");
        let token0 = fixture.contract_hash("cep18_contract_hash_TOKEN_A");
        let token1 = fixture.contract_hash("cep18_contract_hash_TOKEN_B");
        // 1000 tokens
        fixture.transfer(fixture.zero, Key::from(fixture.ali), U256::from(1000), token0);
        fixture.transfer(fixture.zero, Key::from(contract_hash), U256::from(1000000), token0);
        fixture.transfer(fixture.zero, Key::from(contract_hash), U256::from(1000000), token1);
        
        let ali_balance: U256 = fixture.balance_of(Key::from(fixture.ali), "cep18_contract_hash_TOKEN_A");
        assert_eq!(ali_balance, U256::from(1000));
        //fixture.approve(fixture.ali, Key::from(contract_hash), U256::from(1000u128), token0);
        //fixture.swap(fixture.ali, "cep18_contract_hash_TOKEN_A", U256::from(1000u128));
        
        fixture.approve(fixture.ali, Key::from(contract_hash), U256::from(1000), token0);
        //fixture.transfer_from(fixture.bob, Key::from(fixture.zero), Key::from(fixture.ali), U256::from(1000), token0);
        //let ali_balance = fixture.balance_of(Key::from(fixture.ali), "cep18_contract_hash_TOKEN_A");
        //assert_eq!(ali_balance, U256::from(0));

        //let contract_balance_token_a = fixture.balance_of(Key::from(contract_hash), "cep18_contract_hash_TOKEN_A");
        //assert_eq!(contract_balance_token_a, U256::from(1001000));
        //fixture.transfer(fixture.ali, Key::from(fixture.bob), U256::from(1000), token0);
        //fixture.mint(Key::from(fixture.ali), U256::from(1000), token0);
        //fixture.mint(Key::from(contract_hash), U256::from(1000_000_000_000_000_000_000u128), token0);
        //fixture.mint(Key::from(contract_hash), U256::from(1000_000_000_000_000_000_000u128), token1);
        //fixture.approve(fixture.ali, Key::from(contract_hash), U256::from(1000u128), token1);
        fixture.swap(fixture.ali, "cep18_contract_hash_TOKEN_A", U256::from(1000u128));
        let ali_balance = fixture.balance_of(Key::from(fixture.ali), "cep18_contract_hash_TOKEN_A");
        assert_eq!(ali_balance, U256::from(0));
    }
}
*/
fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}