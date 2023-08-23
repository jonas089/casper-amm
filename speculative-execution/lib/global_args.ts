const program = require('commander');

export default function default_args() {
    program
        .option('-secret, --secret-key <string>', 'Path to active secret key', './bin/secret_key.pem')
        .option('-wasm --path-to-wasm <string>', 'Path to contract wasm', './bin/odraAMM.wasm')
        .option('-chain --chain-name <string>', 'Casper network name', 'casper-net-1')
        .option('-payment --payment-amount <number>', 'Payment amount for deploy', 200000000000) //default payment 150 casper
        .option('-host --node-host <string>', 'Node address', 'http://127.0.0.1:25101/rpc')
        .option('-port --node-rpc-port <number>', 'Node rpc port', 25101)
        .option('-lq_token_address, --lq-token-address <string>', 'Address of lq token', '9356c4eadbc9cb5dcbf33a5c02af6a4534b12a03837b8503832ab9d69d3e6fa9')
        .option('-token0_token_address, --token0-address <string>', 'Address of token0', '9356c4eadbc9cb5dcbf33a5c02af6a4534b12a03837b8503832ab9d69d3e6fa9')
        .option('-token1_token_address, --token1-address <string>', 'Address of token1', '9356c4eadbc9cb5dcbf33a5c02af6a4534b12a03837b8503832ab9d69d3e6fa9')
        .option('-odra-package-hash, --package-hash <string>', 'Contract package hash', 'odraContract')
        .option('-odra-allow-override, --allow-override <bool>', 'Allow override', true)
        .option('-odra-allow-upgrade, --allow-upgrade <bool>', 'Allow upgrade', false)
        .option('-bin --bin-path <string>', 'Path to binaries', './bin/')

        // add liquidity args
        .option('-amount0 --amount-0 <number>', 'Amount of token0 to add to LP', 10)
        .option('-amount1 --amount-1 <number>', 'Amount of token1 to add to LP', 10)

        // remove liquidity args
        .option('-shares --shares-count <number>', 'Amount of shares to remove from LP', 1)

        // swap args
        .option('-amount --swap-amount <number>', 'Amount of input token to be swapped for output token', 10)
        .option('-input-hash --input-token-hash <string>', 'Hash of input token', '9356c4eadbc9cb5dcbf33a5c02af6a4534b12a03837b8503832ab9d69d3e6fa9')

        // contract hash with EP
        .option('-contract --contract-hash <string>', 'Hash of the contract to be called', '')
        .parse(process.argv);
    var args = program.opts();
    console.log("Arguments: ", args);
    return args;
}