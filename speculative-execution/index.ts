const { Keys, CasperClient, Contracts, RuntimeArgs, CLValueBuilder, CLKey, CLByteArray } = require("casper-js-sdk");
const fs = require("fs");

/*
1. install 3x token (CEP18)
2. install 3x token (ERC20-concept)
3. mock install contracts
4. install contracts
5. mock execute & execute contracts
*/



function cl_key_constructor(hash_value: string){
    const uint8Array = Uint8Array.from(Buffer.from(hash_value, 'hex'));
    const byteArray = new CLByteArray(uint8Array);
    const key = new CLKey(byteArray);
    return key;
}

export default async function install_contract(nodeHost: string, pathToWasm: string, paymentAmount: number, token: string, token0: string, token1: string){
    const client = new CasperClient(nodeHost);
    const contract = new Contracts.Contract(client);
    const contractWasm = new Uint8Array(fs.readFileSync(pathToWasm).buffer);
    const runtimeArguments = RuntimeArgs.fromMap({
        token: cl_key_constructor(token),
        token0: cl_key_constructor(token0),
        token1: cl_key_constructor(token1)
    });
    const secretKey = "./bin/secret_key.pem";
    const chainId = "casper-net-1";
    const keypair = Keys.Ed25519.loadKeyPairFromPrivateFile(secretKey);
    const deploy = await contract.install(
        contractWasm,
        runtimeArguments,
        paymentAmount.toString(),
        keypair.publicKey,
        chainId,
        [keypair]
    );
    let resp = await client.speculativeDeploy(deploy);
    await console.log(resp);    
}

/*
export async function call_contract(args: any, runtime_args: any){
    const client = new CasperClient(args.nodeHost);
    let contract = new Contracts.Contract(client);
    contract.setContractHash(args.contractHash);
    let keymanager = new KeyManager(args.binPath);
    const req = contract.callEntrypoint(args.entryPoint, runtime_args,  public_key_bytes(args.pubHex), args.chainName, args.paymentAmount, [], 10000000);
    const signedDeploy = DeployUtil.signDeploy(req, keymanager.asymmetricKeyPair());
    const result = signedDeploy.send(args.nodeHost).then((res: any) => {
      console.log("Deploy Result: ", res);
      return res;
    }).catch((error: any) => {
      console.log("Error: ", error);
      return error;
    });
    return result;
}
*/

async function call_install(){
    await install_contract(
        "http://127.0.0.1:25101/rpc",
        "./bin/contract.wasm",
        100000000000, 
        "7c797ed887ca210832a1c26800f63efec6875d0bbf072684bfef097161015e37", 
        "7c797ed887ca210832a1c26800f63efec6875d0bbf072684bfef097161015e37", 
        "7c797ed887ca210832a1c26800f63efec6875d0bbf072684bfef097161015e37"
    );
}
call_install();