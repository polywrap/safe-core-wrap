import * as App from "./wrap";
import {
  PolywrapClient,
  PolywrapClientConfigBuilder,
} from "@polywrap/client-js";
import {
  ethereumWalletPlugin,
  Connection,
  Connections,
} from "@polywrap/ethereum-wallet-js";
import { Wallet, BigNumber } from "ethers";
import { config } from "dotenv";

config();

export const CONNECTION = {
  networkNameOrChainId: "goerli",
};

export const SALT_NONCE = "0x88811188888";

export async function createTransaction(ethers: App.Ethers) {
  const encodedFunction = await ethers.encodeFunction({
    method: "function store(uint256 num) public",
    args: ["99"],
  });
  if (!encodedFunction.ok) throw encodedFunction.error;

  const metaTransactionData = {
    to: "0x56535D1162011E54aa2F6B003d02Db171c17e41e",
    value: "0",
    data: encodedFunction.value,
    operation: 0,
  };

  const gasLimit = await ethers.estimateTransactionGas({
    tx: {
      to: metaTransactionData.to,
      value: metaTransactionData.value,
      data: metaTransactionData.data,
    },
  });

  if (!gasLimit.ok) throw gasLimit.error;
  const TRANSACTION_GAS_BUFFER = 0;
  const withBuffer = BigNumber.from(gasLimit.value).add(TRANSACTION_GAS_BUFFER);
  return { transaction: metaTransactionData, gasLimit: withBuffer.toString() };
}

export function getClient(): PolywrapClient {
  if (!process.env.RPC_URL) {
    throw Error("You must add RPC URL in .env file");
  }

  if (!process.env.PRIVATE_KEY) {
    throw Error("You must add private key in .env file");
  }
  const builder = new PolywrapClientConfigBuilder()
    .addDefaults()
    .setPackages({
      "wrapscan.io/polywrap/ethereum-wallet@1.0": ethereumWalletPlugin({
        connections: new Connections({
          networks: {
            goerli: new Connection({
              provider: process.env.RPC_URL as string,
              signer: new Wallet(process.env.PRIVATE_KEY as string),
            }),
          },
          defaultNetwork: "goerli",
        }),
      }),
    })
    .addEnvs({
      [new App.AccountAbstraction().uri]: {
        connection: CONNECTION,
      },
      [new App.Relayer().uri]: {
        relayerApiKey: "AiaCshYRyAUzTNfZZb8LftJaAl2SS3I8YwhJJXc5J7A_",
      },
    })
    .setRedirects({
      [new App.AccountAbstraction()
        .uri]: `fs/${__dirname}/../../../wraps/account-abstraction-kit/build`,
      [new App.Relayer().uri]: `fs/${__dirname}/../../../wraps/relay-kit/build`,
      [new App.Safe().uri]: `fs/${__dirname}/../../../wraps/protocol-kit/build`,
    });

  const client = new PolywrapClient(builder.build());

  return client;
}
