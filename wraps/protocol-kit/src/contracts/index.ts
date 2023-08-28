import {
  Args_getSafeContractNetworks,
  Args_getVersion,
  ContractNetworksConfig,
  Ethers_Module,
} from "../wrap";
import {
  getFallbackHandlerCompabilityMap,
  getMultisendCallOnlyContractMap,
  getMultisendContractMap,
  getSafeContractMap,
  getSafeFactoryContractMap,
} from "./addresses";

export * from "./utils";

export function getSafeContractAddress(
  safeVersion: string,
  chainId: string,
  isL2: boolean = false
): string {
  const safeContractMap = getSafeContractMap(safeVersion, isL2);

  const hasContractAddress = safeContractMap.has(chainId);

  if (hasContractAddress) {
    const contractAddress = safeContractMap.get(chainId);
    return <string>contractAddress;
  } else {
    throw new Error("No safe contract for provided chainId");
  }
}

export function getSafeFactoryContractAddress(
  safeVersion: string,
  chainId: string
): string {
  const safeFactoryContractMap = getSafeFactoryContractMap(safeVersion);

  const hasContractAddress = safeFactoryContractMap.has(chainId);
  if (hasContractAddress) {
    const contractAddress = safeFactoryContractMap.get(chainId);
    return <string>contractAddress;
  } else {
    throw new Error("No factory contract for provided chainId");
  }
}

export function getMultiSendContractAddress(
  safeVersion: string,
  chainId: string
): string | null {
  const multiSendContractMap = getMultisendContractMap(safeVersion);

  const hasMultisendContractAddress = multiSendContractMap.has(chainId);
  if (hasMultisendContractAddress) {
    return <string>multiSendContractMap.get(chainId);
  } else {
    return null;
  }
}

export function getMultiSendCallOnlyContractAddress(
  safeVersion: string,
  chainId: string
): string | null {
  const multiSendContractMap = getMultisendCallOnlyContractMap(safeVersion);

  const hasMultisendContractAddress = multiSendContractMap.has(chainId);
  if (hasMultisendContractAddress) {
    return <string>multiSendContractMap.get(chainId);
  } else {
    return null;
  }
}

export function getFallbackHandlerCompability(
  safeVersion: string,
  chainId: string
): string | null {
  const fallbackHandlerMap = getFallbackHandlerCompabilityMap(safeVersion);

  const hasFallbackHandler = fallbackHandlerMap.has(chainId);
  if (hasFallbackHandler) {
    return <string>fallbackHandlerMap.get(chainId);
  } else {
    return null;
  }
}

export function getSafeContractNetworks(
  args: Args_getSafeContractNetworks
): ContractNetworksConfig {
  const safeContractVersion = args.version;
  const chainId = args.chainId;
  const isL1Safe: boolean =
    args.isL1Safe != null ? args.isL1Safe!.unwrap() : false;

  if (args.filter == null) {
    return {
      multiSendAddress: getMultiSendContractAddress(
        safeContractVersion,
        chainId.toString()
      ),
      multiSendCallOnlyAddress: getMultiSendCallOnlyContractAddress(
        safeContractVersion,
        chainId.toString()
      ),
      safeMasterCopyAddress: getSafeContractAddress(
        safeContractVersion,
        chainId.toString(),
        !isL1Safe
      ),
      safeProxyFactoryAddress: getSafeFactoryContractAddress(
        safeContractVersion,
        chainId.toString()
      ),
      fallbackHandlerAddress: getFallbackHandlerCompability(
        safeContractVersion,
        chainId.toString()
      ),
    };
  } else {
    let safeContractNetworks: ContractNetworksConfig = {
      multiSendAddress: null,
      multiSendCallOnlyAddress: null,
      safeMasterCopyAddress: null,
      safeProxyFactoryAddress: null,
      fallbackHandlerAddress: null,
    };

    if (args.filter!.multiSendAddress) {
      safeContractNetworks.multiSendAddress = getMultiSendContractAddress(
        safeContractVersion,
        chainId.toString()
      );
    }
    if (args.filter!.multiSendCallOnlyAddress) {
      safeContractNetworks.multiSendCallOnlyAddress =
        getMultiSendCallOnlyContractAddress(
          safeContractVersion,
          chainId.toString()
        );
    }
    if (args.filter!.safeMasterCopyAddress) {
      safeContractNetworks.safeMasterCopyAddress = getSafeContractAddress(
        safeContractVersion,
        chainId.toString(),
        !isL1Safe
      );
    }
    if (args.filter!.safeProxyFactoryAddress) {
      safeContractNetworks.safeProxyFactoryAddress =
        getSafeFactoryContractAddress(safeContractVersion, chainId.toString());
    }
    if (args.filter!.fallbackHandlerAddress) {
      safeContractNetworks.fallbackHandlerAddress =
        getFallbackHandlerCompability(safeContractVersion, chainId.toString());
    }
    return safeContractNetworks;
  }
}

export function getVersion(args: Args_getVersion): string {
  const version = Ethers_Module.callContractView({
    address: args.address,
    method: "function VERSION() public view returns (string)",
    args: [],
    connection: args.connection,
  }).unwrap();

  return version;
}
