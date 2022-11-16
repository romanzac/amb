/**
 * Use this file to configure your truffle project. It's seeded with some
 * common settings for different networks and features like migrations,
 * compilation and testing. Uncomment the ones you need or modify
 * them to suit your project as necessary.
 *
 * More information about configuration can be found at:
 *
 * trufflesuite.com/docs/advanced/configuration
 *
 * To deploy via Infura you'll need a wallet provider (like @truffle/hdwallet-provider)
 * to sign your transactions before they're sent to a remote public node. Infura accounts
 * are available for free at: infura.io/register.
 *
 * You'll also need a mnemonic - the twelve word phrase the wallet uses to generate
 * public/private key pairs. If you're publishing your code to GitHub make sure you load this
 * phrase from a file you've .gitignored so it doesn't accidentally become public.
 *
 */

// create a file at the root of your project and name it .env -- there you can set process variables
// like the mnemomic below. Note: .env is ignored by git in this project to keep your private information safe
require("dotenv").config();
const HDWalletProvider = require("@truffle/hdwallet-provider");

const adminMnemonic = process.env["ADMIN_MNEMONIC"];
const ambMnemonic = process.env["AMB_MNEMONIC"];
const goerliAlchemyKey = process.env["GOERLI_ALCHEMY_KEY"];
const mumbaAlchemyKey = process.env["MUMBAI_ALCHEMY_KEY"];

var privateKeys = [
  adminMnemonic,
  ambMnemonic,
];

module.exports = {
  /**
   * contracts_build_directory tells Truffle where to store compiled contracts
   */
  contracts_build_directory: "./build/contracts",

  /**
   * contracts_directory tells Truffle where to find your contracts
   */
  contracts_directory: "./contracts",

  /**
   * Networks define how you connect to your ethereum client and let you set the
   * defaults web3 uses to send transactions. If you don't specify one truffle
   * will spin up a development blockchain for you on port 9545 when you
   * run `develop` or `test`. You can ask a truffle command to use a specific
   * network from the command line, e.g
   *
   * $ truffle test --network <network-name>
   */

  networks: {
    goerli: {
      network_id: 5,
      provider: new HDWalletProvider(
        privateKeys,
        "https://eth-goerli.g.alchemy.com/v2/" + goerliAlchemyKey,
        0,
        2
      ),
    },
    mumbai: {
      network_id: 80001,
      gas: 5000000,
      provider: new HDWalletProvider(
        privateKeys,
        "https://polygon-mumbai.g.alchemy.com/v2/" + mumbaAlchemyKey,
        0,
        2
      ),
    },

  },

  // Set default mocha options here, use special reporters etc.
  mocha: {
    // timeout: 100000
  },

  // Configure your compilers
  compilers: {
    solc: {
      version: "^0.8.0",       // Fetch exact version from solc-bin (default: truffle's version)
      // docker: true,        // Use "0.5.1" you've installed locally with docker (default: false)
      // settings: {          // See the solidity docs for advice about optimization and evmVersion
      //  optimizer: {
      //    enabled: false,
      //    runs: 200
      //  },
      //  evmVersion: "byzantium"
      // }
    },
  },
  db: {
    enabled: false,
  },
};
