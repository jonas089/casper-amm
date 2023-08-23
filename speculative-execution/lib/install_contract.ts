const { Keys, CasperClient, Contracts, RuntimeArgs, CLValueBuilder, CLKey, CLByteArray, u8 } = require("casper-js-sdk");
const fs = require("fs");

export async function install_contract_speculative(args: any, runtime_args: any){
    const client = new CasperClient(args.nodeHost);
    const contract = new Contracts.Contract(client);
    const contractWasm = new Uint8Array(fs.readFileSync(args.pathToWasm).buffer);
    const runtimeArguments = runtime_args;

    const secretKey = "../bin/secret_key.pem";
    const chainId = "casper-net-1";
    const keypair = Keys.Ed25519.loadKeyPairFromPrivateFile(args.secretKey);
    const deploy = await contract.install(
        contractWasm,
        runtimeArguments,
        args.paymentAmount.toString(),
        keypair.publicKey,
        chainId,
        [keypair]
    );
    let resp = await client.speculativeDeploy(deploy);
    await console.log("Install result: ", resp);    
}