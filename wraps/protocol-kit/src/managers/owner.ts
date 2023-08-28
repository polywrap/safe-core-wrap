import {
  validateAddressIsNotOwner,
  validateAddressIsOwnerAndGetPrev,
  validateOwnerAddress,
  validateThreshold,
} from "../utils/validation";
import {
  Args_encodeAddOwnerWithThresholdData,
  Args_encodeChangeThresholdData,
  Args_encodeRemoveOwnerData,
  Args_encodeSwapOwnerData,
  Env,
  Ethers_Module,
} from "../wrap";
import { BigInt } from "@polywrap/wasm-as";
import * as ContractHelpers from "../contracts";
import { Args_approvedHashes } from "../wrap";

export function encodeAddOwnerWithThresholdData(
  args: Args_encodeAddOwnerWithThresholdData,
  env: Env
): string {
  validateOwnerAddress(args.ownerAddress);
  const owners = ContractHelpers.getOwners({
    safeAddress: env.safeAddress,
    connection: env.connection,
  });
  validateAddressIsNotOwner(args.ownerAddress, owners);
  let threshold: u32 = 0;
  if (args.threshold !== null) {
    threshold = args.threshold!.unwrap();
  } else {
    threshold = ContractHelpers.getThreshold({
      safeAddress: env.safeAddress,
      connection: env.connection,
    });
  }
  validateThreshold(threshold, owners.length + 1);
  const result = Ethers_Module.encodeFunction({
    method:
      "function addOwnerWithThreshold(address owner, uint256 _threshold) public",
    args: [args.ownerAddress, threshold.toString(16)],
  });
  return result.unwrap();
}

export function encodeRemoveOwnerData(
  args: Args_encodeRemoveOwnerData,
  env: Env
): string {
  validateOwnerAddress(args.ownerAddress);
  const owners = ContractHelpers.getOwners({
    safeAddress: env.safeAddress,
    connection: env.connection,
  });
  const prevOwnerAddress = validateAddressIsOwnerAndGetPrev(
    args.ownerAddress,
    owners
  );
  let threshold: u32 = 0;
  if (args.threshold !== null) {
    threshold = args.threshold!.unwrap();
  } else {
    threshold = ContractHelpers.getThreshold({
      safeAddress: env.safeAddress,
      connection: env.connection,
    });
  }
  validateThreshold(threshold, owners.length - 1);
  const result = Ethers_Module.encodeFunction({
    method:
      "function removeOwner(address prevOwner, address owner, uint256 _threshold) public",
    args: [prevOwnerAddress, args.ownerAddress, threshold.toString(16)],
  });
  return result.unwrap();
}

export function encodeSwapOwnerData(
  args: Args_encodeSwapOwnerData,
  env: Env
): string {
  validateOwnerAddress(args.oldOwnerAddress);
  validateOwnerAddress(args.newOwnerAddress);
  const owners = ContractHelpers.getOwners({
    safeAddress: env.safeAddress,
    connection: env.connection,
  });
  validateAddressIsNotOwner(args.newOwnerAddress, owners);
  const prevOwnerAddress = validateAddressIsOwnerAndGetPrev(
    args.oldOwnerAddress,
    owners
  );
  const result = Ethers_Module.encodeFunction({
    method:
      "function swapOwner(address prevOwner, address oldOwner, address newOwner) public",
    args: [prevOwnerAddress, args.oldOwnerAddress, args.newOwnerAddress],
  });
  return result.unwrap();
}

export function encodeChangeThresholdData(
  args: Args_encodeChangeThresholdData,
  env: Env
): string {
  const owners = ContractHelpers.getOwners({
    safeAddress: env.safeAddress,
    connection: env.connection,
  });
  validateThreshold(args.threshold, owners.length);
  const result = Ethers_Module.encodeFunction({
    method: "function changeThreshold(uint256 _threshold) public",
    args: [args.threshold.toString(16)],
  });
  return result.unwrap();
}

export function approvedHashes(args: Args_approvedHashes): BigInt {
  const result = Ethers_Module.callContractView({
    address: args.address,
    method:
      "function approvedHashes(address owner, bytes32 hash) public view returns (uint256)",
    args: [args.ownerAddress, args.hash],
    connection: args.connection,
  }).unwrap();
  return BigInt.from(result);
}
