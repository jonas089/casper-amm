import { CLValue, CLValueBuilder, RuntimeArgs } from "casper-js-sdk";
import { cl_key_constructor } from "./lib/types";
import default_args from "./lib/global_args";
import { call_contract } from './lib/call_entry_point';
const fs = require("fs");
const program = require('commander');

async function add_liquidity() {
    let args = default_args();
    console.log(args);
    if (args.packageHash == 'odraContract'){
        // amount0, amount1 : U512
        let runtime_args = RuntimeArgs.fromMap({
            amount0: CLValueBuilder.u512(args.amount0),
            amount1: CLValueBuilder.u512(args.amount1)
        });
        await call_contract(args, runtime_args);
    }
    else if (args.packageHash == 'ammContract'){
        // amount0, amount1 : U256
        let runtime_args = RuntimeArgs.fromMap({
            amount0: CLValueBuilder.u256(args.amount0),
            amount1: CLValueBuilder.u256(args.amount1)
        });
        await call_contract(args, runtime_args);
    }
}

add_liquidity();