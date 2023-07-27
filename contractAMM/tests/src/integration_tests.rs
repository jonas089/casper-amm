#[cfg(test)]
mod test_fixture;

#[cfg(test)]
mod tests {
    use crate::test_fixture::{Sender, TestEnv};
    use casper_contract::unwrap_or_revert::UnwrapOrRevert;
    use casper_types::{Key, U256};
    #[test]
    fn should_install() {
        let mut fixture: TestEnv = TestEnv::new();
        fixture.install();
    }
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
        
        fixture.approve(fixture.ali, Key::from(fixture.bob), U256::from(1000), token0);
        fixture.transfer_from(fixture.bob, Key::from(fixture.zero), Key::from(fixture.ali), U256::from(1000), token0);
        let ali_balance = fixture.balance_of(Key::from(fixture.ali), "cep18_contract_hash_TOKEN_A");
        assert_eq!(ali_balance, U256::from(0));

        //let contract_balance_token_a = fixture.balance_of(Key::from(contract_hash), "cep18_contract_hash_TOKEN_A");
        //assert_eq!(contract_balance_token_a, U256::from(1001000));
        //fixture.transfer(fixture.ali, Key::from(fixture.bob), U256::from(1000), token0);
        //fixture.mint(Key::from(fixture.ali), U256::from(1000), token0);
        //fixture.mint(Key::from(contract_hash), U256::from(1000_000_000_000_000_000_000u128), token0);
        //fixture.mint(Key::from(contract_hash), U256::from(1000_000_000_000_000_000_000u128), token1);
        //fixture.approve(fixture.ali, Key::from(contract_hash), U256::from(1000u128), token1);
        //fixture.swap(fixture.ali, "cep18_contract_hash_TOKEN_A", U256::from(1000u128));
    }
}

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}