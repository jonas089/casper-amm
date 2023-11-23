rm -rf wasm/contract.wasm
cd ../contract/ && cargo build --release --target wasm32-unknown-unknown && mv target/wasm32-unknown-unknown/release/contract.wasm ../tests/wasm/contract.wasm
