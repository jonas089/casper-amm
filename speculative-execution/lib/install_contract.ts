import { Keys, CasperClient, Contracts, RuntimeArgs, CLValueBuilder, CLKey, CLByteArray } from "casper-js-sdk";
import * as fs from "fs";


export async function install_contract_speculative(args: any, runtime_args: any){
    const client = new CasperClient(args.nodeHost);
    const contract = new Contracts.Contract(client);
    const contractWasm = new Uint8Array(fs.readFileSync(args.pathToWasm).buffer);
    const runtimeArguments = runtime_args;
    const keypair = Keys.Ed25519.loadKeyPairFromPrivateFile(args.secretKey);
    const deploy = await contract.install(
        contractWasm,
        runtimeArguments,
        args.paymentAmount.toString(),
        keypair.publicKey,
        args.chainName,
        [keypair]
    );
    let resp = await client.speculativeDeploy(deploy);
    await console.log("Install result: ", resp);    
}

export async function install_contract(args: any, runtime_args: any){
    const client = new CasperClient(args.nodeHost);
    const contract = new Contracts.Contract(client);
    const contractWasm = new Uint8Array(fs.readFileSync(args.pathToWasm).buffer);
    const runtimeArguments = runtime_args;
    const keypair = Keys.Ed25519.loadKeyPairFromPrivateFile(args.secretKey);
    const deploy = await contract.install(
        contractWasm,
        runtimeArguments,
        args.paymentAmount.toString(),
        keypair.publicKey,
        args.chainName,
        [keypair]
    );
    let resp = await client.putDeploy(deploy);
    await console.log("Install result: ", resp);    
}