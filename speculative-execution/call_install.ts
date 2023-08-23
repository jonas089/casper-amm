import { CLValue, CLValueBuilder, RuntimeArgs } from "casper-js-sdk";
import { cl_key_constructor } from "./lib/types";
import default_args from "./lib/global_args";
import { install_contract_speculative } from "./lib/install_contract";
const fs = require("fs");
const program = require('commander');

async function call_install(){
    let args = default_args();
    let odra_cfg_constructor = CLValueBuilder.string("init");
    console.log(args);
    if (args.packageHash == 'odraContract' && args.pathToWasm == './bin/odraAMM.wasm'){
        let runtime_args = RuntimeArgs.fromMap({
            lq_token_address: cl_key_constructor(args.lqTokenAddress),
            token0_address: cl_key_constructor(args.token0Address),
            token1_address: cl_key_constructor(args.token1Address),
            odra_cfg_package_hash_key_name: CLValueBuilder.string(args.packageHash),
            odra_cfg_allow_key_override: CLValueBuilder.bool(args.allowOverride),
            odra_cfg_is_upgradable: CLValueBuilder.bool(args.allowUpgrade),
            odra_cfg_constructor: CLValueBuilder.string("init")
        });
        await install_contract_speculative(args, runtime_args);
    }
    else if (args.packageHash == 'ammContract' && args.pathToWasm == './bin/casperAMM.wasm'){
        let runtime_args = RuntimeArgs.fromMap({
            token: cl_key_constructor(args.lqTokenAddress),
            token0: cl_key_constructor(args.token0Address),
            token1: cl_key_constructor(args.token1Address)
        });
        await install_contract_speculative(args, runtime_args);
    }
    else if(args.packageHash == 'odraERC' && args.pathToWasm == './bin/odra-erc.wasm'){
        let runtime_args = RuntimeArgs.fromMap({
            name: CLValueBuilder.string(args.odraErcName),
            symbol: CLValueBuilder.string(args.odraErcSymbol),
            decimals: CLValueBuilder.u8(args.odraDecimals),
            initial_supply: CLValueBuilder.u512(args.odraInitialSupply),
            odra_cfg_package_hash_key_name: CLValueBuilder.string(args.packageHash),
            odra_cfg_allow_key_override: CLValueBuilder.bool(args.allowOverride),
            odra_cfg_is_upgradable: CLValueBuilder.bool(args.allowUpgrade),
            odra_cfg_constructor: CLValueBuilder.string("init")
        });
        await install_contract_speculative(args, runtime_args);
    }
}
call_install();