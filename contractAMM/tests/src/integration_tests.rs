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
    fn cross_contract() {
        let mut fixture: TestContext = TestContext::new();
        fixture.install();
        let package_hash =
            fixture.contract_hash_from_named_keys("casper_automated_market_maker_package");
        let token0 = fixture.contract_hash_from_named_keys("cep18_contract_hash_TOKEN_A");
        // let token1 = fixture.contract_hash_from_named_keys("cep18_contract_hash_TOKEN_B");

        let transfer_amount = U256::from(10000);
        let package_key: Key = package_hash.into();

        dbg!(*DEFAULT_ACCOUNT_ADDR);
        dbg!(fixture.ali);
        dbg!(fixture.bob);
        dbg!(package_key);

        // Transfer transfer_amount from installer to ali, ali owns transfer_amount
        fixture.transfer(
            *DEFAULT_ACCOUNT_ADDR,
            Key::from(fixture.ali),
            transfer_amount,
            token0,
        );
        let balance_ali: U256 =
            fixture.balance_of(Key::from(fixture.ali), "cep18_contract_hash_TOKEN_A");
        assert_eq!(balance_ali, transfer_amount);

        // Transfer transfer_amount from installer to AMM package, AMM owns transfer_amount too
        fixture.transfer(*DEFAULT_ACCOUNT_ADDR, package_key, transfer_amount, token0);

        let balance_package: U256 = fixture.balance_of(package_key, "cep18_contract_hash_TOKEN_A");
        assert_eq!(balance_package, transfer_amount);

        // Installer wants to transfer half of the tokens of AMM package to Bob through AMM contract test_transfer so it is the sender for cep-18
        fixture.cross_contract_transfer(
            *DEFAULT_ACCOUNT_ADDR,
            Key::from(fixture.bob),
            transfer_amount / 2,
            token0,
        );

        // Let's check Bob has half the transfer_amount
        let balance_bob: U256 =
            fixture.balance_of(Key::from(fixture.bob), "cep18_contract_hash_TOKEN_A");
        assert_eq!(balance_bob, transfer_amount / 2);

        // Let's check Package has half the transfer_amount left
        let balance_package: U256 = fixture.balance_of(package_key, "cep18_contract_hash_TOKEN_A");
        assert_eq!(balance_package, transfer_amount / 2);
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
        let mut fixture: TestContext = TestContext::new();
        fixture.install();
        let contract_hash = fixture.contract_hash("casper_automated_market_maker");
        let token0 = fixture.contract_hash("cep18_contract_hash_TOKEN_A");
        let token1 = fixture.contract_hash("cep18_contract_hash_TOKEN_B");
        // 1000 tokens
        fixture.transfer(*DEFAULT_ACCOUNT_ADDR, Key::from(fixture.ali), U256::from(1000), token0);
        fixture.transfer(*DEFAULT_ACCOUNT_ADDR, Key::from(contract_hash), U256::from(1000000), token0);
        fixture.transfer(*DEFAULT_ACCOUNT_ADDR, Key::from(contract_hash), U256::from(1000000), token1);

        let ali_balance: U256 = fixture.balance_of(Key::from(fixture.ali), "cep18_contract_hash_TOKEN_A");
        assert_eq!(ali_balance, U256::from(1000));
        //fixture.approve(fixture.ali, Key::from(contract_hash), U256::from(1000u128), token0);
        //fixture.swap(fixture.ali, "cep18_contract_hash_TOKEN_A", U256::from(1000u128));

        fixture.approve(fixture.ali, Key::from(contract_hash), U256::from(1000), token0);
        //fixture.transfer_from(fixture.bob, Key::from(*DEFAULT_ACCOUNT_ADDR), Key::from(fixture.ali), U256::from(1000), token0);
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
