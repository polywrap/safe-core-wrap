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
  Args_approvedHashes,
  Env,
  Ethers_Module,
  Args_getThreshold,
  Args_getOwners,
  Args_isOwner,
} from "../wrap";
import { BigInt } from "@polywrap/wasm-as";
import { JSON } from "assemblyscript-json";

export function encodeAddOwnerWithThresholdData(
  args: Args_encodeAddOwnerWithThresholdData,
  env: Env
): string {
  validateOwnerAddress(args.ownerAddress);
  const owners = getOwners({
    safeAddress: env.safeAddress,
    connection: env.connection,
  });
  validateAddressIsNotOwner(args.ownerAddress, owners);
  let threshold: u32 = 0;
  if (args.threshold !== null) {
    threshold = args.threshold!.unwrap();
  } else {
    threshold = getThreshold({
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
  const owners = getOwners({
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
    threshold = getThreshold({
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
  const owners = getOwners({
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
  const owners = getOwners({
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

export function getThreshold(args: Args_getThreshold): u32 {
  const resp = Ethers_Module.callContractView({
    address: args.safeAddress,
    method: "function getThreshold() public view returns (uint256)",
    args: null,
    connection: args.connection,
  }).unwrap();
  return u32(parseInt(resp, 10));
}

export function getOwners(args: Args_getOwners): string[] {
  const resp = Ethers_Module.callContractView({
    address: args.safeAddress,
    method: "function getOwners() public view returns (address[] memory)",
    args: null,
    connection: args.connection,
  }).unwrap();

  const v = JSON.parse(resp);
  if (!v.isArr) {
    throw new Error("ethereum value is not array: " + v.stringify());
  }
  const arr = (v as JSON.Arr).valueOf();
  const result: string[] = [];
  for (let i = 0; i < arr.length; i++) {
    let s = arr[i];
    if (!s.isString) {
      throw new Error("ethereum value element is not string: " + s.stringify());
    }
    result.push((s as JSON.Str).valueOf());
  }
  return result;
}

export function isOwner(args: Args_isOwner): boolean {
  const resp = Ethers_Module.callContractView({
    address: args.safeAddress,
    method: "function isOwner(address owner) public view returns (bool)",
    args: [args.ownerAddress],
    connection: args.connection,
  }).unwrap();
  if (resp == "true") {
    return true;
  } else {
    return false;
  }
}
