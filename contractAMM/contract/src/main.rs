#![no_std]
#![no_main]
extern crate alloc;
use alloc::{
    string::{String, ToString},
    vec,
    vec::Vec,
};
use casper_contract::{
    contract_api::{
        runtime::{self, put_key},
        storage::{self},
    },
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    account::AccountHash, bytesrepr::ToBytes, contracts::NamedKeys, runtime_args, ApiError, CLType,
    CLTyped, ContractHash, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Key,
    Parameter, RuntimeArgs, URef, U256, ContractPackage, ContractPackageHash
};
mod detail;
mod error;
use crate::detail::get_immediate_caller_address;
use error::Error;

/* Package Hash as Key workaround
   _ (`-.    ('-.               .-. .-')    ('-.                   ('-.   
  ( (OO  )  ( OO ).-.           \  ( OO )  ( OO ).-.             _(  OO)  
 _.`     \  / . --. /   .-----. ,--. ,--.  / . --. /  ,----.    (,------. 
(__...--''  | \-.  \   '  .--./ |  .'   /  | \-.  \  '  .-./-')  |  .---' 
 |  /  | |.-'-'  |  |  |  |('-. |      /,.-'-'  |  | |  |_( O- ) |  |     
 |  |_.' | \| |_.'  | /_) |OO  )|     ' _)\| |_.'  | |  | .--, \(|  '--.  
 |  .___.'  |  .-.  | ||  |`-'| |  .   \   |  .-.  |(|  | '. (_/ |  .--'  
 |  |       |  | |  |(_'  '--'\ |  |\   \  |  | |  | |  '--'  |  |  `---. 
 `--'       `--' `--'   `-----' `--' '--'  `--' `--'  `------'   `------'
*/

#[no_mangle]
pub extern "C" fn initialise(){
    let casper_amm_key: Key = runtime::get_named_arg("casper_amm_key");
    let casper_amm_uref: URef = match runtime::get_key("casper_amm_key"){
        Some(key) => key,
        None => runtime::revert(ApiError::MissingKey)
    }.into_uref().unwrap_or_revert();
    storage::write(casper_amm_uref, casper_amm_key);
}

/* Liquidity Token
          <-.(`-')      (`-')     <-.(`-') <-. (`-')_ 
   <-.     __( OO)      ( OO).->   __( OO)    \( OO) )
 ,--. )   '-'---\_)     /    '._  '-'. ,--.,--./ ,--/ 
 |  (`-')|  .-.  |      |'--...__)|  .'   /|   \ |  | 
 |  |OO )|  | | <-'     `--.  .--'|      /)|  . '|  |)
(|  '__ ||  | |  |         |  |   |  .   ' |  |\    | 
 |     |''  '-'  '-.       |  |   |  |\   \|  | \   | 
 `-----'  `-----'--'       `--'   `--' '--'`--'  `--'
*/

fn _mint(recipient: Key, amount: U256, token_hash: ContractHash) {
    runtime::call_contract::<()>(
        token_hash,
        "mint",
        runtime_args! {
            "owner" => recipient,
            "amount" => amount
        },
    );
}

fn _burn(owner: Key, amount: U256, token_hash: ContractHash) {
    runtime::call_contract::<()>(
        token_hash,
        "burn",
        runtime_args! {
            "owner" => Key::from(owner),
            "amount" => amount
        },
    );
}

fn _update(amm_access_key: Key, reserve0_uref: URef, reserve1_uref: URef, token0_hash: ContractHash, token1_hash: ContractHash){
    let balance0: U256 = runtime::call_contract::<U256>(
        token0_hash,
        "balance_of",
        runtime_args! {
            "address" => amm_access_key,
        }
    );
    let balance1: U256 = runtime::call_contract::<U256>(
        token1_hash,
        "balance_of",
        runtime_args! {
            "address" => amm_access_key,
        }
    );
    storage::write(reserve0_uref, balance0);
    storage::write(reserve1_uref, balance1);   
}

/* Market making logic
   ('-.     _   .-')    _   .-')    
  ( OO ).-.( '.( OO )_ ( '.( OO )_  
  / . --. / ,--.   ,--.),--.   ,--.)
  | \-.  \  |   `.'   | |   `.'   | 
.-'-'  |  | |         | |         | 
 \| |_.'  | |  |'.'|  | |  |'.'|  | 
  |  .-.  | |  |   |  | |  |   |  | 
  |  | |  | |  |   |  | |  |   |  | 
  `--' `--' `--'   `--' `--'   `--'
*/

struct AMM{
    amm_access_key: Key,
    token_hash: ContractHash,
    token0_hash: ContractHash,
    token1_hash: ContractHash,
}

fn collect() -> AMM{
    let amm_access_uref: URef = match runtime::get_key("casper_amm_key"){
        Some(key) => key,
        None => runtime::revert(ApiError::MissingKey)
    }.into_uref().unwrap_or_revert();
    let amm_access_key: Key = storage::read_or_revert(amm_access_uref);
    let token_hash: ContractHash = match runtime::get_key("token") {
        Some(contract_hash) => ContractHash::from(contract_hash.into_hash().unwrap_or_revert()),
        None => runtime::revert(ApiError::MissingKey),
    };
    let token0_hash: ContractHash = match runtime::get_key("token0") {
        Some(contract_hash) => ContractHash::from(contract_hash.into_hash().unwrap_or_revert()),
        None => runtime::revert(ApiError::MissingKey),
    };
    let token1_hash: ContractHash = match runtime::get_key("token1") {
        Some(contract_hash) => ContractHash::from(contract_hash.into_hash().unwrap_or_revert()),
        None => runtime::revert(ApiError::MissingKey),
    };
    AMM {
        amm_access_key: amm_access_key,
        token_hash: token_hash,
        token0_hash: token0_hash,
        token1_hash: token1_hash,
    }
}

#[no_mangle]
pub extern "C" fn swap() {
    let amm_hashs = collect();
    let amm_access_key = amm_hashs.amm_access_key;
    //let token_hash = amm_hashs.token_hash;
    let token0_hash = amm_hashs.token0_hash;
    let token1_hash = amm_hashs.token1_hash;

    let owner: Key = get_immediate_caller_address().unwrap_or_revert();
    let amount: U256 = runtime::get_named_arg("amount");
    let from_token_hash: ContractHash = runtime::get_named_arg("fromToken");
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

    let mut tokenIn = token0_hash;
    let mut tokenOut = token1_hash;
    let mut reserveIn = reserve0;
    let mut reserveOut = reserve1;
    if from_token_hash == token1_hash{
        tokenIn = token1_hash;
        tokenOut = token0_hash;
        reserveIn = reserve1;
        reserveOut = reserve0;
    }
    // transfer tokens to contract
    runtime::call_contract::<()>(
        tokenIn,
        "transfer_from",
        runtime_args! {
            "recipient" => amm_access_key,
            "owner" => owner,
            "amount" => amount
        },
    );
    let amountInWithFee = (amount * 997) / 1000;
    let amountOut = (reserveOut * amountInWithFee) / (reserveIn + amountInWithFee);
    // transfer tokens to contract-caller
    runtime::call_contract::<()>(
        tokenOut,
        "transfer",
        runtime_args! {
            "recipient" => owner,
            "amount" => amountOut
        },
    );
    _update(amm_access_key, reserve0_uref, reserve1_uref, token0_hash, token1_hash);
}

#[no_mangle]
pub extern "C" fn addLiquidity() {
    let amm_hashs = collect();
    let amm_access_key = amm_hashs.amm_access_key;
    let token_hash = amm_hashs.token_hash;
    let token0_hash = amm_hashs.token0_hash;
    let token1_hash = amm_hashs.token1_hash;

    let owner: Key = get_immediate_caller_address().unwrap_or_revert();
    let amount0: U256 = runtime::get_named_arg("amount0");
    let amount1: U256 = runtime::get_named_arg("amount1");

    // transfer funds from caller to contract
    runtime::call_contract::<()>(
        token0_hash,
        "transfer_from",
        runtime_args! {
            "recipient" => amm_access_key,
            "owner" => owner,
            "amount" => amount0
        },
    );
    runtime::call_contract::<()>(
        token1_hash,
        "transfer_from",
        runtime_args! {
            "recipient" => amm_access_key,
            "owner" => owner,
            "amount" => amount1
        },
    );

    // load reserves
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

    // verify contribution
    if reserve0 > U256::zero() || reserve1 > U256::zero(){
        if (reserve0 * amount1) != (reserve1 * amount0){
            runtime::revert(Error::RatioMismatch)
        };
    };

    // current total supply of the liquidity token
    let totalSupply: U256 = runtime::call_contract::<U256>(
        token_hash,
        "total_supply",
        runtime_args! {}
    );
    
    let mut shares: U256 = U256::zero();
    // calculate shares in lp token
    if totalSupply == U256::zero(){
        shares = _sqrt(amount0 * amount1);
    } else {
        shares = _min(
            (amount0 * totalSupply) / reserve0,
            (amount1 * totalSupply) / reserve1
        );
    }
    // mint lp token to caller and update reserves
    _mint(owner, shares, token_hash);
    _update(amm_access_key, reserve0_uref, reserve1_uref, token0_hash, token1_hash);

    /* 
    require(shares > 0, "shares = 0");
    _mint(msg.sender, shares);
    _update(token0.balanceOf(address(this)), token1.balanceOf(address(this)));
    */
}

#[no_mangle]
pub extern "C" fn removeLiquidity() {
    let amm_hashs = collect();
    let amm_access_key = amm_hashs.amm_access_key;
    let token_hash = amm_hashs.token_hash;
    let token0_hash = amm_hashs.token0_hash;
    let token1_hash = amm_hashs.token1_hash;
    // to be implemented
}

/* Math helpers
 _   .-')      ('-.     .-') _    ('-. .-. 
( '.( OO )_   ( OO ).-.(  OO) )  ( OO )  / 
 ,--.   ,--.) / . --. //     '._ ,--. ,--. 
 |   `.'   |  | \-.  \ |'--...__)|  | |  | 
 |         |.-'-'  |  |'--.  .--'|   .|  | 
 |  |'.'|  | \| |_.'  |   |  |   |       | 
 |  |   |  |  |  .-.  |   |  |   |  .-.  | 
 |  |   |  |  |  | |  |   |  |   |  | |  | 
 `--'   `--'  `--' `--'   `--'   `--' `--'

*/

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
fn _min(x: U256, y: U256) -> U256 {
    if x < y {
        x
    } else {
        y
    }
}

/* Call function (init ep)
           (`-')  _                                  <-. (`-')_ 
 _         (OO ).-/    <-.      <-.          <-.        \( OO) )
 \-,-----. / ,---.   ,--. )   ,--. )      (`-')-----.,--./ ,--/ 
  |  .--./ | \ /`.\  |  (`-') |  (`-')    (OO|(_\---'|   \ |  | 
 /_) (`-') '-'|_.' | |  |OO ) |  |OO )     / |  '--. |  . '|  |)
 ||  |OO )(|  .-.  |(|  '__ |(|  '__ |     \_)  .--' |  |\    | 
(_'  '--'\ |  | |  | |     |' |     |'      `|  |_)  |  | \   | 
   `-----' `--' `--' `-----'  `-----'        `--'    `--'  `--' 

*/

#[no_mangle]
pub extern "C" fn call() {
    let mut entry_points: EntryPoints = EntryPoints::new();
    let swap: EntryPoint = EntryPoint::new(
        "swap",
        vec![],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );
    let initialise: EntryPoint  = EntryPoint::new(
        "initialise",
        vec![],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract
    );
    entry_points.add_entry_point(swap);
    entry_points.add_entry_point(initialise);
    let token_hash: Key = runtime::get_named_arg("token");
    let token_hash0: Key = runtime::get_named_arg("token0");
    let token_hash1: Key = runtime::get_named_arg("token1");
    let reserve: Key = storage::new_uref(U256::from(1000u128)).into();

    let mut named_keys = NamedKeys::new();
    named_keys.insert("token".to_string(), token_hash);
    named_keys.insert("token0".to_string(), token_hash0);
    named_keys.insert("token1".to_string(), token_hash1);

    named_keys.insert("reserve0".to_string(), reserve);
    named_keys.insert("reserve1".to_string(), reserve);
    let casper_amm_uref: URef = storage::new_uref("");
    named_keys.insert("casper_amm_key".to_string(), casper_amm_uref.into());

    let package_key_name = "casper_automated_market_maker_package".to_string();
    let (contract_hash, _) = storage::new_contract(
        entry_points,
        Some(named_keys),
        Some(package_key_name),
        Some("amm_access_key".to_string()),
    );
    let contract_hash_key = Key::from(contract_hash);
    runtime::put_key("casper_automated_market_maker", contract_hash_key);
    // workaround to read contract package key from within entry point
    let contract_package_in_runtime: ContractPackageHash = match runtime::get_key("casper_automated_market_maker_package"){
        Some(contract_package) => ContractPackageHash::from(contract_package.into_hash().unwrap_or_revert()),
        None => runtime::revert(ApiError::MissingKey),
    };
    let casper_amm_key: Key = Key::from(contract_package_in_runtime);
    runtime::call_contract::<()>(
        contract_hash,
        "initialise",
        runtime_args! {
            "casper_amm_key" => casper_amm_key
        },
    );
}
