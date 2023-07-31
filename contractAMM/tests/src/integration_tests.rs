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
    fn add_Liquidity(){
        let mut fixture: TestContext = TestContext::new();
        fixture.install();
        let package_key = Key::from(fixture.contract_hash("casper_automated_market_maker_package"));
        // update admin for LP token
        let token = fixture.contract_hash_from_named_keys("cep18_contract_hash_TOKEN2");
        fixture.update_admins(*DEFAULT_ACCOUNT_ADDR, vec![package_key], token);

        let token0 = fixture.contract_hash_from_named_keys("cep18_contract_hash_TOKEN0");
        let token1 = fixture.contract_hash_from_named_keys("cep18_contract_hash_TOKEN1");
        
        fixture.transfer(*DEFAULT_ACCOUNT_ADDR, Key::from(fixture.ali), U256::from(5000), token0);
        fixture.transfer(*DEFAULT_ACCOUNT_ADDR, Key::from(fixture.ali), U256::from(5000), token1);
        fixture.approve(fixture.ali, package_key, U256::from(5000), token0);
        fixture.approve(fixture.ali, package_key, U256::from(5000), token1);
        fixture.add_liquidity(fixture.ali, U256::from(5000), U256::from(5000));
        let balance_package_token0 = fixture.balance_of(package_key, "cep18_contract_hash_TOKEN0");
        let balance_package_token1 = fixture.balance_of(package_key, "cep18_contract_hash_TOKEN1");
        assert_eq!(balance_package_token0, balance_package_token1);
        assert_eq!(balance_package_token0, U256::from(5000));
    }
}

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}
