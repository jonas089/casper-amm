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
mod error;
mod detail;
use crate::detail::get_immediate_caller_address;
mod address;
use crate::address::Address;
use error::Error;

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
pub extern "C" fn initialise(){
    // this endpoint is admin only.

    // set ContractHash
    let contract_key: Key = runtime::get_named_arg("amm_account");
    let contract_key_uref: URef = match runtime::get_key("amm_account"){
        Some(key) => key,
        None => runtime::revert(ApiError::MissingKey)
    }.into_uref().unwrap_or_revert();
    storage::write(contract_key_uref, contract_key);
}

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
    // owner needs to approve this contract as a spender first
    let owner: Address = get_immediate_caller_address().unwrap_or_revert();
    let from_token_hash: ContractHash = runtime::get_named_arg("fromToken");
    let amount: U256 = runtime::get_named_arg("amount");
    if amount <= U256::from(0){
        runtime::revert(Error::ZeroAmount);
    };
    // Get ContractHash of token0
    let token0_uref: URef = match runtime::get_key("token0"){
        Some(key) => key,
        None => runtime::revert(ApiError::MissingKey)
    }.into_uref().unwrap_or_revert();
    let token0_hash: ContractHash = storage::read_or_revert(token0_uref);
    // Get ContractHash of token1
    let token1_uref: URef = match runtime::get_key("token1"){
        Some(key) => key,
        None => runtime::revert(ApiError::MissingKey)
    }.into_uref().unwrap_or_revert();
    let token1_hash: ContractHash = storage::read_or_revert(token1_uref);
    // Check if the input token is valid for this contract
    if from_token_hash != token0_hash && from_token_hash != token1_hash{
        runtime::revert(Error::InvalidToken);
    };
    // Get reserves 
    let reserve0_uref: URef = match runtime::get_key("reserve0"){
        Some(key) => key,
        None => runtime::revert(ApiError::MissingKey)
    }.into_uref().unwrap_or_revert();
    let reserve1_uref: URef = match runtime::get_key("reserve1"){
        Some(key) => key,
        None => runtime::revert(ApiError::MissingKey)
    }.into_uref().unwrap_or_revert();
    let reserve0: U256 = storage::read_or_revert(reserve0_uref);
    let reserve1: U256 = storage::read_or_revert(reserve1_uref);
    // set tokenIn / tokenOut
    let mut tokenIn: ContractHash = token0_hash;
    let mut tokenOut: ContractHash = token1_hash;
    let mut reserveIn: U256 = reserve0;
    let mut reserveOut: U256 = reserve1;
    if from_token_hash == token1_hash{
        tokenIn = token1_hash;
        tokenOut = token0_hash;
        reserveIn = reserve1;
        reserveOut = reserve0;
    };
    // get the key of this contract
    let contract_key_uref: URef = match runtime::get_key("amm_account"){
        Some(key) => key,
        None => runtime::revert(ApiError::MissingKey)
    }.into_uref().unwrap_or_revert();
    let contract_key: Key = storage::read_or_revert(contract_key_uref);
    //emit transfer
    runtime::call_contract::<()>(
        tokenIn,
        "transfer_from",
        runtime_args!{
            "owner" => Key::from(owner),
            "recipient" => contract_key,
            "amount" => amount
        }
    );
    // .3% fee
    let amountInWithFee: U256 = amount * 997 / 1000;
    let amountOut = (reserveOut * amountInWithFee) / (reserveIn + amountInWithFee);
    runtime::call_contract::<()>(
        tokenOut,
        "transfer_from",
        runtime_args!{
            "owner" => contract_key,
            "recipient" => Key::from(owner),
            "amount" => amountOut
        }
    );
    // get token0 and token1 balance of contract and _update()
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
        let initialise: EntryPoint = EntryPoint::new(
            "initialise",
            vec![],
            CLType::Unit,
            EntryPointAccess::Public,
            EntryPointType::Contract
        );
        let swap: EntryPoint = EntryPoint::new(
            "swap",
            vec![],
            CLType::Unit,
            EntryPointAccess::Public,
            EntryPointType::Contract
        );
        entry_points.add_entry_point(initialise);
        entry_points.add_entry_point(swap);
        entry_points
    };
    let named_keys = {
        let mut named_keys = NamedKeys::new();
        let tokenHash0: ContractHash = runtime::get_named_arg("token0");
        let tokenHash1: ContractHash = runtime::get_named_arg("token1");
        let tokenHash: ContractHash = runtime::get_named_arg("token");

        let contract_key: URef = storage::new_uref("amm_account");
        let token0: URef = storage::new_uref("token0");
        storage::write(token0, tokenHash0);
        let token1: URef = storage::new_uref("token1");
        storage::write(token1, tokenHash1);
        let token: URef = storage::new_uref("token");
        storage::write(token, tokenHash);
        let reserve0: URef = storage::new_uref("reserve0");
        storage::write(reserve0, U256::from(1_000_000_000_000_000_000_000u128));
        let reserve1: URef = storage::new_uref("reserve1");
        storage::write(reserve1, U256::from(1_000_000_000_000_000_000_000u128));

        named_keys.insert("token0".to_string(), token0.into());
        named_keys.insert("token1".to_string(), token1.into());
        named_keys.insert("token".to_string(), token.into());
        named_keys.insert("reserve0".to_string(), reserve0.into());
        named_keys.insert("reserve1".to_string(), reserve1.into());

        named_keys.insert("amm_account".to_string(), contract_key.into());

        named_keys
    };
    let (contract_hash, contract_version) = storage::new_contract(
        entry_points,
        Some(named_keys),
        Some("casper_automated_market_maker".to_string()),
        Some("casper_amm_key".to_string())
    );
    runtime::put_key("casper_automated_market_maker", Key::from(contract_hash));
    runtime::call_contract::<()>(
        contract_hash,
        "initialise",
        runtime_args!{
            "amm_account" => Key::from(contract_hash)
        }
    );
}