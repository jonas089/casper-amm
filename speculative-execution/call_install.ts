import { CLValue, CLValueBuilder, RuntimeArgs } from "casper-js-sdk";
import { cl_key_constructor } from "./lib/types";
import default_args from "./lib/global_args";
import { install_contract_speculative } from "./lib/install_contract";
const fs = require("fs");
const program = require('commander');

async function call_install(){
    let install_args = default_args();
    let odra_cfg_constructor = CLValueBuilder.string("init");
    console.log(install_args);
    if (install_args.packageHash == 'odraContract' && install_args.pathToWasm == './bin/odraAMM.wasm'){
        let runtime_args = RuntimeArgs.fromMap({
            lq_token_address: cl_key_constructor(install_args.lqTokenAddress),
            token0_address: cl_key_constructor(install_args.token0Address),
            token1_address: cl_key_constructor(install_args.token1Address),
            odra_cfg_package_hash_key_name: CLValueBuilder.string(install_args.packageHash),
            odra_cfg_allow_key_override: CLValueBuilder.bool(install_args.allowOverride),
            odra_cfg_is_upgradable: CLValueBuilder.bool(install_args.allowUpgrade),
            odra_cfg_constructor: CLValueBuilder.string("init")
        });
        await install_contract_speculative(install_args, runtime_args);
    }
    else if (install_args.packageHash == 'ammContract' && install_args.pathToWasm == './bin/casperAMM.wasm'){
        let runtime_args = RuntimeArgs.fromMap({
            token: cl_key_constructor(install_args.lqTokenAddress),
            token0: cl_key_constructor(install_args.token0Address),
            token1: cl_key_constructor(install_args.token1Address)
        });
        await install_contract_speculative(install_args, runtime_args);
    }
}
call_install();
/*

odra_cfg_package_hash_key_name: odra_cfg_package_hash_key_name,
odra_cfg_allow_key_override: odra_cfg_allow_key_override,
odra_cfg_is_upgradable: odra_cfg_is_upgradable,
odra_cfg_constructor: CLValueBuilder.string("init")
*/