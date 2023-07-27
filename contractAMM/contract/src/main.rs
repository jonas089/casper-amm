#![no_std]
#![no_main]
extern crate alloc;
use alloc::{
    string::{String, ToString},
    vec,
    vec::Vec
};
use casper_types::{
    bytesrepr::{ToBytes},
    account::AccountHash, contracts::NamedKeys, runtime_args, ApiError, CLType,
    EntryPoint, EntryPointAccess, ContractHash,
    EntryPointType, EntryPoints, Key, Parameter, RuntimeArgs, URef, U256, CLTyped,
};
use casper_contract::{
    contract_api::{
        runtime,
        storage::{self},
    },
    unwrap_or_revert::UnwrapOrRevert,
};
mod address;
use crate::address::Address;


// experimental square root function
fn _sqrt(y: U256) -> U256 {
    let mut z: U256 = U256::from(0);
    if y >= U256::from(3) {
        z = y;
        let mut x: U256 = y / U256::from(2) + U256::from(1);
        while x <= z {
            z = x;
            x = (y / x + x) / U256::from(2);
        }
    } else if y != U256::from(0) {
        z = U256::from(1);
    }
    return z;
}

// custom min function
fn _min(x: U256, y: U256) -> U256{
    if x < y{
        x
    } else{
        y
    }
}

/*
fn _call(){
    runtime::call_contract::<()>(
        contract_hash, 
        "mint_stablecoin", 
        runtime_args!{
            "recipient" => Key::from(runtime::get_caller()),
            "amount" => U256::from(1)
        }
    );
}
*/
#[no_mangle]
pub extern "C" fn _mint(){
    let recipient: Address = runtime::get_named_arg("recipient");
    let amount: U256 = runtime::get_named_arg("amount");
    let token_uref: URef = match runtime::get_key("token"){
        Some(key) => key,
        None => runtime::revert(ApiError::MissingKey)
    }.into_uref().unwrap_or_revert();
    let token_hash: ContractHash = storage::read_or_revert(token_uref);
    
    // mint token at ContractHash
    runtime::call_contract::<()>(
        token_hash,
        "mint",
        runtime_args!{
            "owner" => Key::from(recipient),
            "amount" => amount
        }
    );
}

#[no_mangle]
pub extern "C" fn _burn(){
    let owner: Address = runtime::get_named_arg("owner");
    let amount: U256 = runtime::get_named_arg("amount");
    let token_uref: URef = match runtime::get_key("token"){
        Some(key) => key,
        None => runtime::revert(ApiError::MissingKey)
    }.into_uref().unwrap_or_revert();
    let token_hash: ContractHash = storage::read_or_revert(token_uref);

    // burn token at ContractHash
    runtime::call_contract::<()>(
        token_hash,
        "burn",
        runtime_args!{
            "owner" => Key::from(owner),
            "amount" => amount
        }
    );
}

#[no_mangle]
pub extern "C" fn _update(){
    let new_reserve0: U256 = runtime::get_named_arg("reserve0");
    let new_reserve1: U256 = runtime::get_named_arg("reserve1");
    let reserve0_uref: URef = match runtime::get_key("reserve0"){
        Some(key) => key,
        None => runtime::revert(ApiError::MissingKey)
    }.into_uref().unwrap_or_revert();
    let reserve1_uref: URef = match runtime::get_key("reserve1"){
        Some(key) => key,
        None => runtime::revert(ApiError::MissingKey)
    }.into_uref().unwrap_or_revert();
    storage::write(reserve0_uref, new_reserve0);
    storage::write(reserve1_uref, new_reserve1);
}

#[no_mangle]
pub extern "C" fn swap(){

}

#[no_mangle]
pub extern "C" fn addLiquidity(){

}

#[no_mangle]
pub extern "C" fn removeLiquidity(){

}

#[no_mangle]
pub extern "C" fn call(){
    let entry_points: EntryPoints = {
        let mut entry_points: EntryPoints = EntryPoints::new();


        entry_points
    };
    let named_keys = {
        let mut named_keys = NamedKeys::new();
        let contractAddressKey: Key = runtime::get_named_arg("contractAddress");
        let tokenHash0: ContractHash = runtime::get_named_arg("token0");
        let tokenHash1: ContractHash = runtime::get_named_arg("token1");
        let tokenHash: ContractHash = runtime::get_named_arg("token");

        let contractAddress: URef = storage::new_uref("contractAddress");
        storage::write(contractAddress, contractAddressKey);
        let token0: URef = storage::new_uref("token0");
        storage::write(token0, tokenHash0);
        let token1: URef = storage::new_uref("token1");
        storage::write(token1, tokenHash1);
        let token: URef = storage::new_uref("token");
        storage::write(token, tokenHash);
        let reserve0: URef = storage::new_uref("reserve0");
        storage::write(reserve0, U256::from(0));
        let reserve1: URef = storage::new_uref("reserve1");
        storage::write(reserve1, U256::from(0));

        named_keys
    };
    let (contract_hash, contract_version) = storage::new_contract(
        entry_points,
        Some(named_keys),
        Some("casper_automated_market_maker".to_string()),
        Some("casper_amm_key".to_string())
    );
    runtime::put_key("casper_automated_market_maker", Key::from(contract_hash));
}