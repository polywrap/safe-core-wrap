import * as App from "./types/wrap";
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
import { dateTimePlugin } from "@polywrap/datetime-plugin-js";
import {
  abi as factoryAbi_1_2_0,
  bytecode as factoryBytecode_1_2_0,
} from "@gnosis.pm/safe-contracts_1.2.0/build/contracts/GnosisSafeProxyFactory.json";

import {
  abi as factoryAbi_1_3_0,
  bytecode as factoryBytecode_1_3_0,
} from "@gnosis.pm/safe-contracts_1.3.0/build/artifacts/contracts/proxies/GnosisSafeProxyFactory.sol/GnosisSafeProxyFactory.json";

import {
  abi as safeAbi_1_2_0,
  bytecode as safeBytecode_1_2_0,
} from "@gnosis.pm/safe-contracts_1.2.0/build/contracts/GnosisSafe.json";

import {
  abi as safeAbi_1_3_0,
  bytecode as safeBytecode_1_3_0,
} from "@gnosis.pm/safe-contracts_1.3.0/build/artifacts/contracts/GnosisSafe.sol/GnosisSafe.json";

export enum DEPLOYMENT_VERSIONS {
  "1.2.0",
  "1.3.0",
}

export type VERSIONS = keyof typeof DEPLOYMENT_VERSIONS;
export type Address = string;
export type ContractInfo = {
  abi: string;
  bytecode: string;
};

export enum SUPPORTED_CONTRACTS {
  SAFE = "SAFE",
  FACTORY = "FACTORY",
  SAFE_L2 = "SAFE_L2",
  FALLBACK_HANDLER = "FALLBACK_HANDLER",
  MULTISEND = "MULTISEND",
  MULTISEND_CALL_ONLY = "MULTISEND_CALL_ONLY",
}

export type OptionalKeys<T> = { [K in keyof T]?: T[K] };
export type SupportedContracts<T> = OptionalKeys<
  Record<keyof typeof SUPPORTED_CONTRACTS, Record<VERSIONS, T>>
>;

export const SAFE_VERSIONS_INFO: SupportedContracts<ContractInfo> = {
  SAFE: {
    ["1.2.0"]: {
      abi: JSON.stringify(safeAbi_1_2_0),
      bytecode: safeBytecode_1_2_0,
    },
    ["1.3.0"]: {
      abi: JSON.stringify(safeAbi_1_3_0),
      bytecode: safeBytecode_1_3_0,
    },
  },
  FACTORY: {
    ["1.2.0"]: {
      abi: JSON.stringify(factoryAbi_1_2_0),
      bytecode: factoryBytecode_1_2_0,
    },
    ["1.3.0"]: {
      abi: JSON.stringify(factoryAbi_1_3_0),
      bytecode: factoryBytecode_1_3_0,
    },
  },
};

export const CONNECTION = { networkNameOrChainId: "testnet" };

export async function getClient(): Promise<PolywrapClient> {
  const signer = new Wallet(
    "0x4f3edf983ac636a65a842ce7c78d9aa706d3b113bce9c46f30d7d21715b23b1d"
  );
  const config = new PolywrapClientConfigBuilder()
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
      "wrapscan.io/polywrap/datetime@1.0.0": dateTimePlugin({}),
    })
    .setRedirect(
      "wrapscan.io/polywrap/protocol-kit@0.1.0",
      `fs/${__dirname}/../build`
    );
  return new PolywrapClient(config.build());
}

// @TODO(cbrzn): This can be completely dynamic (Rather than hard coding the version variables)
export async function setUpContracts(
  client: PolywrapClient
): Promise<SupportedContracts<Address>> {
  const ethers = new App.Ethers(client);

  const safe_v120 = SAFE_VERSIONS_INFO.SAFE!["1.2.0"];
  const factory_v120 = SAFE_VERSIONS_INFO.FACTORY!["1.2.0"];

  const safe_v130 = SAFE_VERSIONS_INFO.SAFE!["1.3.0"];
  const factory_v130 = SAFE_VERSIONS_INFO.FACTORY!["1.3.0"];

  let safeAddressV1_2_0: Address;
  let safeAddressV1_3_0: Address;
  let factoryAddressV1_2_0: Address;
  let factoryAddressV1_3_0: Address;

  // Set up v1.2.0 contracts
  const safeDeploy_v120 = await ethers.deployContract({
    ...safe_v120,
    connection: CONNECTION,
  });
  if (!safeDeploy_v120.ok) throw safeDeploy_v120.error;
  safeAddressV1_2_0 = safeDeploy_v120.value;

  const safeFactoryContractResponse_v120 = await ethers.deployContract({
    ...factory_v120,
    connection: CONNECTION,
  });

  if (!safeFactoryContractResponse_v120.ok)
    throw safeFactoryContractResponse_v120.error;
  factoryAddressV1_2_0 = safeFactoryContractResponse_v120.value;

  // Set up v1.3.0 contracts
  const deploy_v130 = await ethers.deployContract({
    ...safe_v130,
    connection: CONNECTION,
  });

  if (!deploy_v130.ok) throw deploy_v130.error;
  safeAddressV1_3_0 = deploy_v130.value;

  const safeFactoryContractResponse_v130 = await ethers.deployContract({
    ...factory_v130,
    connection: CONNECTION,
  });

  if (!safeFactoryContractResponse_v130.ok)
    throw safeFactoryContractResponse_v130.error;
  factoryAddressV1_3_0 = safeFactoryContractResponse_v130.value;

  return {
    SAFE: {
      ["1.2.0"]: safeAddressV1_2_0,
      ["1.3.0"]: safeAddressV1_3_0,
    },
    FACTORY: {
      ["1.2.0"]: factoryAddressV1_2_0,
      ["1.3.0"]: factoryAddressV1_3_0,
    },
  };
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
