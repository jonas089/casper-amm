//const { CasperClient, Contracts, DeployUtil } = require("casper-js-sdk");
//const { public_key_bytes } = require('./types');
//const { KeyManager } = require('./keymanager');
import { KeyManager } from "./keymanager";
import { public_key_bytes } from "./types";
import { CasperClient, Contracts, DeployUtil } from "casper-js-sdk";

export async function call_contract(args: any, runtime_args: any){
  const client = new CasperClient(args.nodeHost);
  let contract = new Contracts.Contract(client);
  contract.setContractHash(args.contractHash);
  let keymanager = new KeyManager(args.binPath);
  const req = contract.callEntrypoint(args.entryPoint, runtime_args,  public_key_bytes(args.pubHex), args.chainName, args.paymentAmount, [], 10000000);
  const signedDeploy = DeployUtil.signDeploy(req, keymanager.asymmetricKeyPair());
  let resp = await signedDeploy.send(args.nodeHost);
  await console.log("Deploy Result: ", resp);
  return resp;
}

export async function call_contract_speculative(args: any, runtime_args: any){
  const client = new CasperClient(args.nodeHost);
  let contract = new Contracts.Contract(client);
  contract.setContractHash(args.contractHash);
  let keymanager = new KeyManager(args.binPath);
  const req = contract.callEntrypoint(args.entryPoint, runtime_args,  public_key_bytes(args.pubHex), args.chainName, args.paymentAmount, [], 10000000);
  const signedDeploy = DeployUtil.signDeploy(req, keymanager.asymmetricKeyPair());
  let resp = await client.speculativeDeploy(signedDeploy);
  await console.log("Install result: ", resp); 
  return resp;
}