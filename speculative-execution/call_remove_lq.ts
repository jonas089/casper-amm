import { CLValue, CLValueBuilder, RuntimeArgs } from "casper-js-sdk";
import { cl_key_constructor } from "./lib/types";
import default_args from "./lib/global_args";
import { call_contract, call_contract_speculative } from './lib/call_entry_point';

async function add_liquidity() {
    let args = default_args();
    console.log(args);
    if (args.packageHash == 'odraContract'){
        // shares : U512
        let runtime_args = RuntimeArgs.fromMap({
            shares: CLValueBuilder.u512(args.sharesCount),
        });
        if (args.isSpeculativeExecution == 'false'){
            await call_contract(args, runtime_args);
        }
        else{
            await call_contract_speculative(args, runtime_args);
        }
    }
    else if (args.packageHash == 'ammContract'){
        // shares : U256
        let runtime_args = RuntimeArgs.fromMap({
            shares: CLValueBuilder.u256(args.sharesCount),
        });
        if (args.isSpeculativeExecution == 'false'){
            await call_contract(args, runtime_args);
        }
        else{
            await call_contract_speculative(args, runtime_args);
        }
    }
}

add_liquidity();