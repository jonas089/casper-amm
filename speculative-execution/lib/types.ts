const { CLValueBuilder, CLAccountHash, CLByteArray, CLKey, CLPublicKey } = require("casper-js-sdk");

// construct account hash CLValue, no prefix
export function account_hash_constructor(account_hash: string){
    const uint8Array = Uint8Array.from(Buffer.from(account_hash, 'hex'));
    const byteArray = new CLAccountHash(uint8Array);
    const key = CLValueBuilder.key(byteArray);
    return key;
}

// construct (contract) hash CLValue, no prefix
export function cl_key_constructor(hash_value: string){
    const uint8Array = Uint8Array.from(Buffer.from(hash_value, 'hex'));
    const byteArray = new CLByteArray(uint8Array);
    const key = new CLKey(byteArray);
    return key;
}

// derive any hash type and construct a CLValue hash
export function serialize_any_hash(hash_value: string){
    if (hash_value.startsWith("account")){
        return account_hash_constructor(hash_value.substring(13));
    }
    else if (hash_value.startsWith("hash")){
        return cl_key_constructor(hash_value.substring(5));
    }
    else{
        return cl_key_constructor(hash_value);
    }
}

// Hex-Key to Byte-Key
export function public_key_bytes(hex_key: string){
    return CLPublicKey.fromHex(hex_key);
}