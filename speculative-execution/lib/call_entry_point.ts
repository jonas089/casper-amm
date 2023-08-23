const { CasperClient, Contracts, DeployUtil } = require("casper-js-sdk");
const { public_key_bytes } = require('./types');
const { KeyManager } = require('./keymanager');

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

export async function call_contract_speculative(args: any, runtime_args: any){

}