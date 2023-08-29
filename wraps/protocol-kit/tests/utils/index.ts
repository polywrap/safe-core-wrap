import { ETH_ENS_IPFS_MODULE_CONSTANTS, runCli } from "@polywrap/cli-js";
import {
  PolywrapClient,
  PolywrapClientConfigBuilder,
} from "@polywrap/client-js";
import {
  ethereumWalletPlugin,
  Connection,
  Connections,
} from "@polywrap/ethereum-wallet-js";
import { Wallet } from "ethers";
import * as App from "../types/wrap";
import { setupAccounts } from "./setup";

export * from "./setup";

export const CONNECTION = { networkNameOrChainId: "testnet" };

export function getClient(config?: {
  signer?: Wallet;
  env?: Record<string, unknown>;
}): PolywrapClient {
  let signer = config?.signer;

  if (!signer) {
    const account = setupAccounts();
    signer = account[0].signer;
  }
  const builder = new PolywrapClientConfigBuilder()
    .addDefaults()
    .setPackages({
      "wrapscan.io/polywrap/ethereum-wallet@1.0": ethereumWalletPlugin({
        connections: new Connections({
          networks: {
            testnet: new Connection({
              provider: ETH_ENS_IPFS_MODULE_CONSTANTS.ethereumProvider,
              signer,
            }),
          },
          defaultNetwork: "testnet",
        }),
      }),
    })
    .setRedirect(
      "wrapscan.io/polywrap/protocol-kit@0.1.0",
      `fs/${__dirname}/../../build`
    );

  if (config?.env) {
    builder.addEnv("wrapscan.io/polywrap/protocol-kit@0.1.0", config.env);
  }
  return new PolywrapClient(builder.build());
}

export async function initInfra(): Promise<void> {
  const { exitCode, stderr, stdout } = await runCli({
    args: ["infra", "up", "--verbose", "--modules", "eth-ens-ipfs"],
  });

  if (exitCode) {
    throw Error(
      `initInfra failed to start test environment.\nExit Code: ${exitCode}\nStdErr: ${stderr}\nStdOut: ${stdout}`
    );
  }

  await new Promise<void>(function (resolve) {
    setTimeout(() => resolve(), 5000);
  });
}

export async function stopInfra(): Promise<void> {
  const { exitCode, stderr, stdout } = await runCli({
    args: ["infra", "down", "--verbose", "--modules", "eth-ens-ipfs"],
  });

  if (exitCode) {
    throw Error(
      `initInfra failed to stop test environment.\nExit Code: ${exitCode}\nStdErr: ${stderr}\nStdOut: ${stdout}`
    );
  }
}

export const createTransaction = async (
  data?: App.Safe_SafeTransactionDataPartial
) => {
  let safe = new App.Safe(getClient());
  const defaults = {
    data: "0x",
    to: "0xFFcf8FDEE72ac11b5c542428B35EEF5769C409f0",
    value: "500000000000000000", // 0.5 ETH
  };
  const result = await safe.createTransaction({
    tx: {
      ...defaults,
      ...data,
    },
  });
  if (!result.ok) {
    throw "Error creating transaction: " + result.error;
  }

  return result.value;
};

export const fundSafeBalance = async (
  safeAddress: string,
  amount = "1000000000000000000" // 1 ETH
) => {
  let ethers = new App.Ethers(getClient());
  const result = await ethers.sendTransactionAndWait({
    tx: {
      to: safeAddress,
      value: amount,
      data: "0x",
    },
    connection: CONNECTION,
  });

  if (!result.ok) {
    throw "Error funding safe: " + result.error;
  }
};
