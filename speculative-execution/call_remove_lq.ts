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
        // shares : U512
        let runtime_args = RuntimeArgs.fromMap({
            shares: CLValueBuilder.u512(args.sharesCount),
        });
        await call_contract(args, runtime_args);
    }
    else if (args.packageHash == 'ammContract'){
        // shares : U256
        let runtime_args = RuntimeArgs.fromMap({
            shares: CLValueBuilder.u256(args.sharesCount),
        });
        await call_contract(args, runtime_args);
    }
}

add_liquidity();