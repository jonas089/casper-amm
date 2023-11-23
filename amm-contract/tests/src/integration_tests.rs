mod test_fixture;

#[cfg(test)]
mod tests {
    use crate::test_fixture::TestContext;
    use casper_engine_test_support::DEFAULT_ACCOUNT_ADDR;
    use casper_types::{Key, U256};

    // Install 3 cep-18 tokens with default account and install AMM contract
    #[test]
    fn should_install() {
        let mut fixture: TestContext = TestContext::new();
        fixture.install();
    }
    
    #[test]
    fn swap(){
        let mut fixture: TestContext = TestContext::new();
        fixture.install();
        let package_key = Key::from(fixture.contract_hash("casper_automated_market_maker_package"));
        // update admin for LP token
        let token = fixture.contract_hash_from_named_keys("cep18_contract_hash_TOKEN2");
        fixture.update_admins(*DEFAULT_ACCOUNT_ADDR, vec![package_key], token);
        // define token contract hashs
        let token0 = fixture.contract_hash_from_named_keys("cep18_contract_hash_TOKEN0");
        let token1 = fixture.contract_hash_from_named_keys("cep18_contract_hash_TOKEN1");
        // fund ali and provide liquidity
        fixture.transfer(*DEFAULT_ACCOUNT_ADDR, Key::from(fixture.ali), U256::from(5000), token0);
        fixture.transfer(*DEFAULT_ACCOUNT_ADDR, Key::from(fixture.ali), U256::from(5000), token1);
        // allow contract to spend 5000 of each token
        fixture.approve(fixture.ali, package_key, U256::from(5000), token0);
        fixture.approve(fixture.ali, package_key, U256::from(5000), token1);
        // add liquidity
        fixture.add_liquidity(fixture.ali, U256::from(5000), U256::from(5000));
        // check liquidity
        let balance_package_token0 = fixture.balance_of(package_key, "cep18_contract_hash_TOKEN0");
        let balance_package_token1 = fixture.balance_of(package_key, "cep18_contract_hash_TOKEN1");
        assert_eq!(balance_package_token0, balance_package_token1);
        assert_eq!(balance_package_token0, U256::from(5000));
        /*State
            ratio = 1, liquidity = 5000, fee = 0.3%
        */

        // fund bob with 1000 token0
        fixture.transfer(*DEFAULT_ACCOUNT_ADDR, Key::from(fixture.bob), U256::from(1000), token0);
        // approve contract to spend 1000 token0 in bobs name
        fixture.approve(fixture.bob, package_key, U256::from(1000), token0);
        // swap 1000 token0 for 997 token1
        fixture.swap(fixture.bob, "cep18_contract_hash_TOKEN0", U256::from(1000));
        let bob_token0 = fixture.balance_of(Key::from(fixture.bob), "cep18_contract_hash_TOKEN0");
        let bob_token1 = fixture.balance_of(Key::from(fixture.bob), "cep18_contract_hash_TOKEN1");
        assert_eq!(bob_token0, U256::from(0));
        /*AmountOut
            amountWithFee = amountIn * 997 / 1000
            (reserveOut * amountWithFee / reserveIn + amountWithFee)
        */
        assert_eq!(bob_token1, U256::from(831));
        // check reserve
        let package_token0 = fixture.balance_of(package_key, "cep18_contract_hash_TOKEN0");
        let package_token1 = fixture.balance_of(package_key, "cep18_contract_hash_TOKEN1");
        assert_eq!(package_token0, U256::from(6000));
        assert_eq!(package_token1, U256::from(4169));
    }

    #[test]
    fn liquidity(){
        let mut fixture: TestContext = TestContext::new();
        fixture.install();
        let package_key = Key::from(fixture.contract_hash("casper_automated_market_maker_package"));
        // update admin for LP token
        let token = fixture.contract_hash_from_named_keys("cep18_contract_hash_TOKEN2");
        fixture.update_admins(*DEFAULT_ACCOUNT_ADDR, vec![package_key], token);
        // define token contract hashs
        let token0 = fixture.contract_hash_from_named_keys("cep18_contract_hash_TOKEN0");
        let token1 = fixture.contract_hash_from_named_keys("cep18_contract_hash_TOKEN1");
        // fund ali accont
        fixture.transfer(*DEFAULT_ACCOUNT_ADDR, Key::from(fixture.ali), U256::from(5000), token0);
        fixture.transfer(*DEFAULT_ACCOUNT_ADDR, Key::from(fixture.ali), U256::from(5000), token1);
        // approve the contract to spend ali's funds
        fixture.approve(fixture.ali, package_key, U256::from(5000), token0);
        fixture.approve(fixture.ali, package_key, U256::from(5000), token1);
        // add liquidity to the contract
        fixture.add_liquidity(fixture.ali, U256::from(5000), U256::from(5000));
        // check balances
        let mut package_token0 = fixture.balance_of(package_key, "cep18_contract_hash_TOKEN0");
        let mut package_token1 = fixture.balance_of(package_key, "cep18_contract_hash_TOKEN1");
        assert_eq!(package_token0, package_token1);
        assert_eq!(package_token0, U256::from(5000));
        // check ali's shares in the LP
        let ali_shares = fixture.balance_of(Key::from(fixture.ali), "cep18_contract_hash_TOKEN2");
        assert_eq!(ali_shares, U256::from(5000));
        // approve the contract to spend ali's shares and remove liquidity
        fixture.approve(fixture.ali, package_key, U256::from(5000), token);
        fixture.remove_liquidity(fixture.ali, U256::from(5000));
        // check balances of contract
        let package_shares = fixture.balance_of(package_key, "cep18_contract_hash_TOKEN2");
        package_token0 = fixture.balance_of(package_key, "cep18_contract_hash_TOKEN0");
        package_token1 = fixture.balance_of(package_key, "cep18_contract_hash_TOKEN1");
        assert_eq!(package_shares, U256::zero());
        assert_eq!(package_token0, U256::zero());
        assert_eq!(package_token1, U256::zero());
        // check balances of ali
        let ali_shares = fixture.balance_of(Key::from(fixture.ali), "cep18_contract_hash_TOKEN2");
        let ali_token0 = fixture.balance_of(Key::from(fixture.ali), "cep18_contract_hash_TOKEN0");
        let ali_token1 = fixture.balance_of(Key::from(fixture.ali), "cep18_contract_hash_TOKEN1");
        assert_eq!(ali_shares, U256::zero());
        assert_eq!(ali_token0, U256::from(5000));
        assert_eq!(ali_token1, U256::from(5000));
    }
}

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}
