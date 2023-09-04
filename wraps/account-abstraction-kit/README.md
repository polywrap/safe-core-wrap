# Account Abstraction Kit Wrap

The Account Abstraction Wrap is an implementation of the [Account Abstraction Kit](https://github.com/safe-global/safe-core-sdk/tree/main/packages/account-abstraction-kit) in the form of a Polywrap wrap.


## Configuration
You will need to add the [ethereum wallet plugin](https://github.com/polywrap/ethereum-wallet) to the Polywrap Client's config manually. You can configure this using the [Polywrap Client Config Builder](https://docs.polywrap.io/tutorials/use-wraps/configure-client):

```javascript
import {
  ethereumWalletPlugin,
  Connection,
  Connections,
} from "@polywrap/ethereum-wallet-js";
import { PolywrapClient, ClientConfigBuilder } from "@polywrap/client-js";

const builder = new ClientConfigBuilder();

builder
  .addDefaults()
  .addPackages({
    // This adds the ethereum provider (wallet) plugin
    "wrap://wrascan.io/polywrap/ethereum-wallet@1": ethereumWalletPlugin({
      connections: new Connections({
        networks: {
          goerli: new Connection({        
            provider: "RPC_URL", // Add your RPC URL here
            signer: new Wallet("PRIVATE_KEY"), // Add your private key here
            
            // Alternatively, instead of setting provider and signer, you can simply use window.ethereum as your provider:
            // provider: window.ethereum
          }),
        },
        defaultNetwork: "goerli",
      }),
    }),
  })
  // This URL should be replaced with the Wrap URL of the AA Wrap
  .addEnv("wrapscan.io/polywrap/account-abstraction-kit@0.1", {
    connection: {
      networkNameOrChainId: "goerli", // Determines which network you're using
    },
  });

// Build the config
const config = builder.build();

// You can now use the Polywrap Client to invoke the AA Wrap
const client = new PolywrapClient(config);

const invocationResult = client.invoke({
  // This URI should be replaced with the Wrap URI of the AA Wrap
  uri: "wrapscan.io/polywrap/account-abstraction-kit@0.1",
  method: "relayTransaction",
  args: { ... }
})
```

## Run!

With your client successfully configured, you can now run any function on the Account Abstraction Wrap with ease.
See the examples below, or [take a look at the Safe AA wrap tests](https://github.com/polywrap/safe-core-wrap/tree/main/wraps/account-abstraction-kit/tests) for invocation examples.

## Examples

### Get Safe Address

Since your Safe address is deterministic based on your wallet's address and a nonce, you can determine it ahead-of-time:

```javascript
const addressResult = await client.invoke({
  // This URI should be replaced with the Wrap URI of the AA Wrap
  uri: "wrapscan.io/polywrap/account-abstraction-kit@0.1",
  method: "getSafeAddress",
  args: {
    config: {
      saltNonce: "0x2588023872383771" // Replace this with your own salt nonce
    }
  }
});
```

### Relay Sponsored Transaction

Let's assume we have a smart contract with a `store(uint256 num)` function deployed to the address `0x56535D1162011E54aa2F6B003d02Db171c17e41e`.
If we want to, for example, invoke this function, we first need to encode it using our [Ethers Wrap](https://github.com/polywrap/ethers):

```javascript
const encodeResult = await client.invoke({
  uri: "wrapscan.io/polywrap/ethers@1.1",
  method: "encodeFunction",
  args: {
    method: "function store(uint256 num) public",
    args: [
      "14"
    ]
  }
});
```

The Account Abstraction Wrap's `relayTransaction` method allows you to make a transaction by performing the following steps:

1. It checks whether there's a deployed Safe contract on your predicted address.
2. It deploys a Safe contract to the predicted address if there isn't one.
3. It executes the transaction using the Gelato Relay.

```javascript
await client.invoke({
  // This URI should be replaced with the Wrap URI of the AA Wrap
  uri: "wrapscan.io/polywrap/account-abstraction-kit@0.1",
  method: "relayTransaction",
  args: {
    transaction: {
      to: "0x56535D1162011E54aa2F6B003d02Db171c17e41e", // Address of the contract you're invoking
      value: "0",
      data: encodeResult.data, // Invocation data.
      operation: "0"
    },
    options: {
      gasLimit: "300000",
      isSponsored: true // Determines whether this should be a sponsored (gasless) transaction or not
    },
    config: {
      // Replace this with your own salt nonce. This is used to deploy your safe contract if there is none.
      // This is not required. If not supplied, a default nonce will be used.
      saltNonce: "0x2588023872383771"
    }
  }
});
```

## Support

For any questions or problems related to the Safe AA wrap or Polywrap at large, please visit our [Discord](https://discord.polywrap.io).
