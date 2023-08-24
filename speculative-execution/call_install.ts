import { CLValue, CLValueBuilder, RuntimeArgs } from "casper-js-sdk";
import { cl_key_constructor } from "./lib/types";
import default_args from "./lib/global_args";
import { install_contract_speculative, install_contract } from "./lib/install_contract";

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
        if (args.isSpeculativeExecution == 'false'){
            await install_contract(args, runtime_args);
        }
        else{
            await install_contract_speculative(args, runtime_args);
        }
    }
    else if (args.packageHash == 'ammContract' && args.pathToWasm == './bin/casperAMM.wasm'){
        let runtime_args = RuntimeArgs.fromMap({
            token: cl_key_constructor(args.lqTokenAddress),
            token0: cl_key_constructor(args.token0Address),
            token1: cl_key_constructor(args.token1Address)
        });
        if (args.isSpeculativeExecution == 'false'){
            await install_contract(args, runtime_args);
        }
        else{
            await install_contract_speculative(args, runtime_args);
        }
    }
    else if(args.packageHash.startsWith('odraERC') && args.pathToWasm == './bin/odra-erc.wasm'){
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
        if (args.isSpeculativeExecution == 'false'){
            await install_contract(args, runtime_args);
        }
        else{
            await install_contract_speculative(args, runtime_args);
        }
    }
}
call_install();

/* Install Erc20 (odra) - speculative
ts-node call_install.ts --path-to-wasm ./bin/odra-erc.wasm  --odra-erc-name token --odra-erc-symbol tkn --is-speculative-execution true --package-hash odraERC-1
*/

/* Install Erc20 (odra) - actual
ts-node call_install.ts --path-to-wasm ./bin/odra-erc.wasm  --odra-erc-name token --odra-erc-symbol tkn --is-speculative-execution false --node-host http://127.0.0.1:11101/rpc --node-rpc-port 11101 --package-hash odraERC-1
*/

/* Install odra contract - speculative
ts-node call_install.ts --path-to-wasm ./bin/odraAMM.wasm --lq-token-address [TOKEN_ADDRESS] --token0-address [TOKEN_ADDRESS] --token1-address [TOKEN_ADDRESS] --is-speculative-execution true --package-hash odraContract
*/

/* Install odra contract - actual
ts-node call_install.ts --path-to-wasm ./bin/odraAMM.wasm --lq-token-address d4de5b35f3478eab086c396faea1afdda46960eeabf0e043c6cd3dddd00e271b --token0-address d463a506a2979c1b56083124b28c7dfe7cec84d4c462033ff43be9d27ae5d22a --token1-address efeb4bcb1f503d44a92a9e37f392960bc3247dad8db655c4fff6d773be57e88e --is-speculative-execution false --package-hash odraContract --node-host http://127.0.0.1:11101/rpc --node-rpc-port 11101
*/