import { CLValue, CLValueBuilder, RuntimeArgs } from "casper-js-sdk";
import { cl_key_constructor, serialize_any_hash } from "./lib/types";
import default_args from "./lib/global_args";
import { call_contract, call_contract_speculative } from './lib/call_entry_point';

async function call_mint() {
    let args = default_args();
    console.log(args);
    let runtime_args = RuntimeArgs.fromMap({
        address: serialize_any_hash(args.odraErcMintAddr),
        amount: CLValueBuilder.u256(args.odraErcMintAmount)
    });
    if (args.isSpeculativeExecution == 'false'){
        await call_contract(args, runtime_args);
    }
    else{
        await call_contract_speculative(args, runtime_args);
    };
}

call_mint();