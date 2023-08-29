import { Wallet } from "ethers";
import * as App from "../types/wrap";

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

import {
  abi as multisendAbi_1_2_0,
  bytecode as multisendBytecode_1_2_0,
} from "@gnosis.pm/safe-contracts_1.2.0/build/contracts/MultiSend.json";
import {
  abi as multisendAbi_1_3_0,
  bytecode as multisendBytecode_1_3_0,
} from "@gnosis.pm/safe-contracts_1.3.0/build/artifacts/contracts/libraries/MultiSend.sol/MultiSend.json";

import {
  abi as multisendCallOnlyAbi,
  bytecode as multisendCallOnlyBytecode,
} from "@gnosis.pm/safe-contracts_1.3.0/build/artifacts/contracts/libraries/MultiSendCallOnly.sol/MultiSendCallOnly.json";

import {
  abi as ERC20MintableAbi,
  bytecode as ERC20MintableBytecode,
} from "./ERC20Mock.json";

import { CONNECTION } from ".";

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
  MULTISEND: {
    ["1.2.0"]: {
      abi: JSON.stringify(multisendAbi_1_2_0),
      bytecode: multisendBytecode_1_2_0,
    },
    ["1.3.0"]: {
      abi: JSON.stringify(multisendAbi_1_3_0),
      bytecode: multisendBytecode_1_3_0,
    },
  },
  MULTISEND_CALL_ONLY: {
    ["1.2.0"]: {
      abi: JSON.stringify(multisendCallOnlyAbi),
      bytecode: multisendCallOnlyBytecode,
    },
    ["1.3.0"]: {
      abi: JSON.stringify(multisendCallOnlyAbi),
      bytecode: multisendCallOnlyBytecode,
    },
  },
};

// @TODO(cbrzn): This can be completely dynamic (Rather than hard coding the version variables)
export async function setUpContracts(
  ethers: App.Ethers
): Promise<SupportedContracts<Address>> {
  const safeV1_2_0 = SAFE_VERSIONS_INFO.SAFE!["1.2.0"];
  const factoryV1_2_0 = SAFE_VERSIONS_INFO.FACTORY!["1.2.0"];

  const safeV1_3_0 = SAFE_VERSIONS_INFO.SAFE!["1.3.0"];
  const factoryV1_3_0 = SAFE_VERSIONS_INFO.FACTORY!["1.3.0"];

  const multisendV1_2_0 = SAFE_VERSIONS_INFO.MULTISEND!["1.2.0"];
  const multisendV1_3_0 = SAFE_VERSIONS_INFO.MULTISEND!["1.3.0"];
  const multisendCallOnly = SAFE_VERSIONS_INFO.MULTISEND_CALL_ONLY!["1.3.0"];

  let safeAddressV1_2_0: Address;
  let safeAddressV1_3_0: Address;
  let factoryAddressV1_2_0: Address;
  let factoryAddressV1_3_0: Address;
  let multisendAddressV1_2_0: Address;
  let multisendAddressV1_3_0: Address;
  let multisendCallOnlyAddress: Address;

  // Set up v1.2.0 contracts
  const safeMasterCopyDeploy_v120 = await ethers.deployContract({
    ...safeV1_2_0,
    connection: CONNECTION,
  });
  if (!safeMasterCopyDeploy_v120.ok) throw safeMasterCopyDeploy_v120.error;
  safeAddressV1_2_0 = safeMasterCopyDeploy_v120.value;

  const safeFactoryContractResponse_v120 = await ethers.deployContract({
    ...factoryV1_2_0,
    connection: CONNECTION,
  });

  if (!safeFactoryContractResponse_v120.ok)
    throw safeFactoryContractResponse_v120.error;
  factoryAddressV1_2_0 = safeFactoryContractResponse_v120.value;

  const multisendDeployContractResponseV1_2_0 = await ethers.deployContract({
    ...multisendV1_2_0,
    connection: CONNECTION,
  });

  if (!multisendDeployContractResponseV1_2_0.ok)
    throw multisendDeployContractResponseV1_2_0.error;
  multisendAddressV1_2_0 = multisendDeployContractResponseV1_2_0.value;

  // Set up v1.3.0 contracts
  const safeMasterCopydeployV1_3_0 = await ethers.deployContract({
    ...safeV1_3_0,
    connection: CONNECTION,
  });

  if (!safeMasterCopydeployV1_3_0.ok) throw safeMasterCopydeployV1_3_0.error;
  safeAddressV1_3_0 = safeMasterCopydeployV1_3_0.value;

  const safeFactoryContractResponseV1_3_0 = await ethers.deployContract({
    ...factoryV1_3_0,
    connection: CONNECTION,
  });

  if (!safeFactoryContractResponseV1_3_0.ok)
    throw safeFactoryContractResponseV1_3_0.error;
  factoryAddressV1_3_0 = safeFactoryContractResponseV1_3_0.value;

  const multisendDeployContractResponseV1_3_0 = await ethers.deployContract({
    ...multisendV1_3_0,
    connection: CONNECTION,
  });

  if (!multisendDeployContractResponseV1_3_0.ok)
    throw multisendDeployContractResponseV1_3_0.error;
  multisendAddressV1_3_0 = multisendDeployContractResponseV1_3_0.value;

  const multisendCallOnlyDeployResponse = await ethers.deployContract({
    ...multisendCallOnly,
    connection: CONNECTION,
  });

  if (!multisendCallOnlyDeployResponse.ok)
    throw multisendCallOnlyDeployResponse.error;
  multisendCallOnlyAddress = multisendCallOnlyDeployResponse.value;

  return {
    SAFE: {
      ["1.2.0"]: safeAddressV1_2_0,
      ["1.3.0"]: safeAddressV1_3_0,
    },
    FACTORY: {
      ["1.2.0"]: factoryAddressV1_2_0,
      ["1.3.0"]: factoryAddressV1_3_0,
    },
    MULTISEND: {
      ["1.2.0"]: multisendAddressV1_2_0,
      ["1.3.0"]: multisendAddressV1_3_0,
    },
    MULTISEND_CALL_ONLY: {
      ["1.2.0"]: multisendCallOnlyAddress,
      ["1.3.0"]: multisendCallOnlyAddress,
    },
  };
}

export const setupAccounts = () => {
  return [
    {
      signer: new Wallet(
        "0x4f3edf983ac636a65a842ce7c78d9aa706d3b113bce9c46f30d7d21715b23b1d"
      ),
      address: "0x90F8bf6A479f320ead074411a4B0e7944Ea8c9C1",
    },
    {
      signer: new Wallet(
        "0x6cbed15c793ce57650b9877cf6fa156fbef513c4e6134f022a85b1ffdd59b2a1"
      ),
      address: "0xFFcf8FDEE72ac11b5c542428B35EEF5769C409f0",
    },
    {
      signer: new Wallet(
        "0x6370fd033278c143179d81c5526140625662b8daa446c22ee2d73db3707e620c"
      ),
      address: "0x22d491Bde2303f2f43325b2108D26f1eAbA1e32b",
    },
  ];
};

export const deployTestSafe = async (
  safe: App.Safe,
  input: App.Safe_DeploymentInput
) => {
  const deploySafeResponse = await safe.deploySafe({
    input: {
      ...input,
      safeDeploymentConfig: {
        saltNonce: Date.now().toString(),
        ...input.safeDeploymentConfig,
      },
    },
  });

  if (!deploySafeResponse.ok) {
    throw "Error deploying test safe: " + deploySafeResponse.error;
  }

  return deploySafeResponse.value;
};

export const deployTestErc20 = async (ethers: App.Ethers) => {
  const deployErc20Response = await ethers.deployContract({
    abi: JSON.stringify(ERC20MintableAbi),
    bytecode: ERC20MintableBytecode,
    connection: CONNECTION,
  });
  if (!deployErc20Response.ok) {
    throw "Error deploying test ERC20: " + deployErc20Response.error;
  }

  return deployErc20Response.value;
};
