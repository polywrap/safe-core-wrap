import {
  PolywrapClient,
  PolywrapClientConfigBuilder,
} from "@polywrap/client-js";
import * as App from "../types/wrap";
import {
  ethereumWalletPlugin,
  Connection,
  Connections,
} from "@polywrap/ethereum-wallet-js";
import { Wallet } from "ethers";
import { config } from "dotenv";

config();
export const CONNECTION = {
  networkNameOrChainId: "goerli",
};

export const SALT_NONCE = "0x3219329299299";

export function getClient(): PolywrapClient {
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
      [new App.AccountAbstraction().uri]: `fs/${__dirname}/../../build`,
      [new App.Relayer().uri]: `fs/${__dirname}/../../../relay-kit/build`,
      [new App.Safe().uri]: `fs/${__dirname}/../../../protocol-kit/build`,
    });

  const client = new PolywrapClient(builder.build());

  return client;
}
