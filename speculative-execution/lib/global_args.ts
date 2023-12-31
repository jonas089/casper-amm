const program = require('commander');

export default function default_args() {
    program
        .option('-secret, --secret-key <string>', 'Path to active secret key', './bin/secret_key.pem')
        .option('-wasm --path-to-wasm <string>', 'Path to contract wasm', './bin/odraAMM.wasm')
        .option('-chain --chain-name <string>', 'Casper network name', 'casper-net-1')
        .option('-payment --payment-amount <number>', 'Payment amount for deploy', 200000000000)
        .option('-host --node-host <string>', 'Node address', 'http://127.0.0.1:25101/rpc')
        .option('-port --node-rpc-port <number>', 'Node rpc port', 25101)
        .option('-lq_token_address, --lq-token-address <string>', 'Address of lq token', '')
        .option('-token0_token_address, --token0-address <string>', 'Address of token0', '')
        .option('-token1_token_address, --token1-address <string>', 'Address of token1', '')
        .option('-odra-package-hash, --package-hash <string>', 'Contract package hash', 'odraContract')
        .option('-odra-allow-override, --allow-override <bool>', 'Allow override', true)
        .option('-odra-allow-upgrade, --allow-upgrade <bool>', 'Allow upgrade', false)
        .option('-bin --bin-path <string>', 'Path to binaries', './bin/')
        .option('-pub-key --pub-hex <string>', 'Public key hex', '01a7c0b45926fa9c919aca8467c82417e5cfc7444d0b065d5b2906a21ffa1c366e')

        // add liquidity args
        .option('-amount0 --amount-0 <number>', 'Amount of token0 to add to LP', 100000)
        .option('-amount1 --amount-1 <number>', 'Amount of token1 to add to LP', 100000)

        // remove liquidity args
        .option('-shares --shares-count <number>', 'Amount of shares to remove from LP', 1)

        // swap args
        .option('-amount --swap-amount <number>', 'Amount of input token to be swapped for output token', 10)
        .option('-input-hash --input-token-hash <string>', 'Address of input token', '9356c4eadbc9cb5dcbf33a5c02af6a4534b12a03837b8503832ab9d69d3e6fa9')

        // contract hash with EP
        .option('-contract --contract-hash <string>', 'Hash of the contract to be called', 'hash-f3863553fdb8658b4dae98b1d199b045c96523c447d41fb91f0947f60c61a70b')
        .option('-ep --entry-point <string>', 'Name of the entry point to be called', 'init')


        // odra Erc20 install args
        .option('-odra-erc-name --odra-erc-name <string>', 'Name of the odra erc20 token to be installed', 'odra_token')
        .option('-odra-erc-symbol --odra-erc-symbol <string>', 'Symbol of the odra erc20 token to be installed', 'odratkn')
        .option('-odra-decimals --odra-decimals <number>', 'Decimals of the token', 18)
        .option('-odra-init-supply --odra-initial-supply <number>', 'Initial supply of odra token U512', 0)
        
        // odra Erc20 mint args
        .option('-odra-erc-m-addr --odra-erc-mint-addr <string>', 'Account / contract hash to mint token to', '')
        .option('-odra-erc-m-amount --odra-erc-mint-amount <number>', 'Amount to mint to Account', 10)

        // speculative bool
        .option('-is-speculative --is-speculative-execution <bool>', 'Set whether to execute or speculative execute', true)
        .parse(process.argv);
    var args = program.opts();
    console.log("Arguments: ", args);
    return args;
}