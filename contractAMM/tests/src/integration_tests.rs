#[cfg(test)]
mod test_fixture;

#[cfg(test)]
mod tests {
    use crate::test_fixture::{Sender, TestEnv};
    use blake2::digest::{FixedOutput, consts::{U25, U2}, FixedOutputDirty};
    use casper_contract::unwrap_or_revert::UnwrapOrRevert;
    use casper_types::{Key, U256};
    #[test]
    fn should_install() {
        let mut fixture: TestEnv = TestEnv::new();
        fixture.install();
    }
}

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}