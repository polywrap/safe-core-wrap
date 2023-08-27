import { SafeFactory } from ".";
import {
  EMPTY_DATA,
  PREDETERMINED_SALT_NONCE,
  ZERO_ADDRESS,
} from "../constants";
import { getSafeContractNetworks } from "../contracts";
import {
  Ethers_Connection,
  Ethers_Module,
  SafeAccountConfig,
  SafeDeploymentConfig,
  CustomContract,
  DeploymentPayload,
} from "../wrap";
import { BigInt, Result, JSON, Box, wrap_debug_log } from "@polywrap/wasm-as";

export const validateSafeAccountConfig = (config: SafeAccountConfig): void => {
  if (config.owners.length <= 0)
    throw new Error("Owner list must have at least one owner");

  const threshold = config.threshold;

  if (threshold) {
    if (threshold <= 0)
      throw new Error("Threshold must be greater than or equal to 1");
    if (threshold > <u32>config.owners.length)
      throw new Error("Threshold must be lower than or equal to owners length");
  }
};

export const validateSafeDeploymentConfig = (
  config: SafeDeploymentConfig
): void => {
  if (BigInt.from(config.saltNonce).lt(0))
    throw new Error("saltNonce must be greater than or equal to 0");
};

export function encodeSetupCallData(accountConfig: SafeAccountConfig): string {
  const args: string[] = [];

  args.push(JSON.from(accountConfig.owners).stringify());

  const threshold = accountConfig.threshold.toString();

  args.push(<string>threshold);

  if (accountConfig.to != null) {
    args.push(accountConfig.to!);
  } else {
    args.push(ZERO_ADDRESS);
  }
  if (accountConfig.data != null) {
    args.push(accountConfig.data!);
  } else {
    args.push(EMPTY_DATA);
  }
  if (accountConfig.fallbackHandler != null) {
    args.push(accountConfig.fallbackHandler!);
  } else {
    args.push(ZERO_ADDRESS);
  }
  if (accountConfig.paymentToken != null) {
    args.push(accountConfig.paymentToken!);
  } else {
    args.push(ZERO_ADDRESS);
  }
  if (accountConfig.payment) {
    args.push(accountConfig.payment!.toString());
  } else {
    args.push("0");
  }
  if (accountConfig.paymentReceiver != null) {
    args.push(accountConfig.paymentReceiver!);
  } else {
    args.push(ZERO_ADDRESS);
  }

  return Ethers_Module.encodeFunction({
    method:
      "function setup(address[] _owners,uint256 _threshold,address to,bytes data,address fallbackHandler,address paymentToken,uint256 payment,address paymentReceiver)",
    args: args,
  }).unwrap();
}

export function isContractDeployed(
  address: string,
  connection: Ethers_Connection | null
): boolean {
  const code = Ethers_Module.sendRpc({
    method: "eth_getCode",
    connection: connection,
    params: [address, "pending"],
  }).unwrap();

  if (code != null) {
    return code != "0x";
  }
  return false;
}

export function getInitCode(
  safeProxyFactoryAddr: string,
  gnosisSafeAddr: string,
  connection: Ethers_Connection | null
): Result<string, string> {
  const proxyCreationCode = SafeFactory.proxyCreationCode({
    address: safeProxyFactoryAddr,
    connection: connection,
  });

  const constructorData = Ethers_Module.encodeParams({
    types: ["address"],
    values: [gnosisSafeAddr],
  });
  if (constructorData.isErr) {
    return constructorData;
  }

  return Result.Ok<string, string>(
    proxyCreationCode + constructorData.unwrap().slice(2)
  );
}

export function generateSalt(
  nonce: string,
  initializer: string
): Result<string, string> {
  // const bigIntNonce = BigInt.fromString(nonce, 16);
  const saltNonce = Ethers_Module.encodeParams({
    types: ["uint256"],
    values: [BigInt.fromString(nonce).toString()],
  });
  if (saltNonce.isErr) {
    return saltNonce;
  }
  let initializerHash = Ethers_Module.keccak256({ value: initializer });
  if (initializerHash.isErr) {
    return Result.Err<string, string>(initializerHash.unwrapErr());
  }

  let initHash = initializerHash.unwrap();

  return Ethers_Module.keccak256({
    value: Ethers_Module.solidityPack({
      values: [initHash + saltNonce.unwrap().slice(2)],
      types: ["bytes"],
    }).unwrap(),
  });
}

export function prepareSafeDeployPayload(
  safeAccountConfig: SafeAccountConfig,
  safeDeploymentConfig: SafeDeploymentConfig | null,
  customContractAddresses: CustomContract | null,
  connection: Ethers_Connection | null
): DeploymentPayload {
  validateSafeAccountConfig(safeAccountConfig);
  if (safeDeploymentConfig != null) {
    validateSafeDeploymentConfig(safeDeploymentConfig);
  }

  let saltNonce: string = "";
  let safeContractVersion: string = "1.3.0";
  let isL1Safe = false;

  // TODO: handle partial config, fallback on each option separately
  if (safeDeploymentConfig != null) {
    if (safeDeploymentConfig.saltNonce != null) {
      saltNonce = safeDeploymentConfig.saltNonce;
    }
    if (safeDeploymentConfig.version != null) {
      safeContractVersion = safeDeploymentConfig.version!;
    }
    if (safeDeploymentConfig.isL1Safe) {
      isL1Safe = true;
    }
  } else {
    saltNonce = PREDETERMINED_SALT_NONCE;
    safeContractVersion = "1.3.0";
  }

  const chainId = Ethers_Module.getChainId({ connection }).unwrap();
  let safeContractAddress: string = "";
  let safeFactoryContractAddress: string = "";

  if (customContractAddresses != null) {
    if (customContractAddresses.proxyFactoryContract != null) {
      safeFactoryContractAddress =
        customContractAddresses.proxyFactoryContract!;
    }
    if (customContractAddresses.safeFactoryContract != null) {
      safeContractAddress = customContractAddresses.safeFactoryContract!;
    }
  }

  if (safeContractAddress == "") {
    const contracts = getSafeContractNetworks({
      version: safeContractVersion,
      chainId: chainId.toString(),
      isL1Safe: Box.from(isL1Safe),
      filter: {
        safeMasterCopyAddress: true,
        safeProxyFactoryAddress: false,
        multiSendAddress: false,
        multiSendCallOnlyAddress: false,
        fallbackHandlerAddress: false,
      },
    });
    safeContractAddress = contracts.safeMasterCopyAddress!;
  }

  if (safeFactoryContractAddress == "") {
    const contracts = getSafeContractNetworks({
      version: safeContractVersion,
      chainId: chainId.toString(),
      isL1Safe: Box.from(isL1Safe),
      filter: {
        safeMasterCopyAddress: false,
        safeProxyFactoryAddress: true,
        multiSendAddress: false,
        multiSendCallOnlyAddress: false,
        fallbackHandlerAddress: false,
      },
    });
    safeFactoryContractAddress = contracts.safeProxyFactoryAddress!;
  }
  if (safeAccountConfig.fallbackHandler == null) {
    const contracts = getSafeContractNetworks({
      version: safeContractVersion,
      chainId: chainId.toString(),
      isL1Safe: Box.from(isL1Safe),
      filter: {
        safeMasterCopyAddress: false,
        safeProxyFactoryAddress: false,
        multiSendAddress: false,
        multiSendCallOnlyAddress: false,
        fallbackHandlerAddress: true,
      },
    });
    safeAccountConfig.fallbackHandler = contracts.fallbackHandlerAddress;
  }

  const initializer = encodeSetupCallData(safeAccountConfig);
  return {
    initializer,
    saltNonce,
    safeFactoryContractAddress,
    safeContractAddress,
  };
}
