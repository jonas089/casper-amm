import { CLValue, CLValueBuilder, RuntimeArgs } from "casper-js-sdk";
import { cl_key_constructor } from "./lib/types";
import default_args from "./lib/global_args";
import { call_contract, call_contract_speculative } from './lib/call_entry_point';
const fs = require("fs");
const program = require('commander');

async function add_liquidity() {
    let args = default_args();
    console.log(args);
    if (args.packageHash == 'odraContract'){
        // amount : U512
        let runtime_args = RuntimeArgs.fromMap({
            amount: CLValueBuilder.u512(args.swapAmount),
            fromToken: cl_key_constructor(args.inputTokenHash)
        });
        if (args.isSpeculativeExecution == 'false'){
            await call_contract(args, runtime_args);
        }
        else{
            await call_contract_speculative(args, runtime_args);
        }
    }
    else if (args.packageHash == 'ammContract'){
        // amount : U256
        let runtime_args = RuntimeArgs.fromMap({
            amount: CLValueBuilder.u256(args.swapAmount),
            fromToken: cl_key_constructor(args.inputTokenHash)
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