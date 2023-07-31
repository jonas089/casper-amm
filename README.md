# R&D AMM
This is an automated market maker that has been translated from Solidity into Rust.

## Run unit tests
```
git clone git@github.com:jonas089/casperAMM
cd casperAMM/contractAMM/test
./compile.sh
cargo test
```
This will run a limited scope of test cases against add_liquidity, remove_liquidity and swap

## Estimate gas (_sqrt)
Casper V1.5 - speculative execution endpoint