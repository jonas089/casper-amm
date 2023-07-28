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
        let package_hash =
        fixture.contract_hash("casper_automated_market_maker_package");
        let token0 = fixture.contract_hash_from_named_keys("cep18_contract_hash_TOKEN_A");
        let package_key: Key = Key::from(package_hash);
        fixture.transfer(*DEFAULT_ACCOUNT_ADDR, Key::from(fixture.ali), U256::from(5000), token0);
        fixture.approve(fixture.ali, package_key, U256::from(1000), token0);
        fixture.swap(fixture.ali, "cep18_contract_hash_TOKEN_A", U256::from(1000));
        let balance_contract = fixture.balance_of(package_key, "cep18_contract_hash_TOKEN_A");
        assert_eq!(
            balance_contract,
            U256::from(1000)
        );
        let balance_ali = fixture.balance_of(Key::from(fixture.ali), "cep18_contract_hash_TOKEN_A");
        assert_eq!(
            balance_ali,
            U256::from(4000)
        )
    }
}

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}
