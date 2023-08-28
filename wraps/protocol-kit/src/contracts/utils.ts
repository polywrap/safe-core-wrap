import { BigInt, Box } from "@polywrap/wasm-as";
import {
  SafeSignature,
  SafeTransactionData,
  OperationType,
  Ethers_Module,
  Args_getThreshold,
  Args_getOwners,
  Args_isOwner,
  Args_getTransactionHash,
  Env,
  Args_signTransactionHash,
  SafeTransactionDataPartial,
} from "../wrap";
import { ZERO_ADDRESS } from "../constants";
import { JSON } from "assemblyscript-json";
import { adjustVInSignature, arrayify } from "../utils/signature";

export function signTransactionHash(
  args: Args_signTransactionHash,
  env: Env
): SafeSignature {
  const signer = Ethers_Module.getSignerAddress({
    connection: env.connection,
  }).unwrap();

  const byteArray = arrayify(args.hash).buffer;

  const signature = Ethers_Module.signMessageBytes({
    bytes: byteArray,
    connection: {
      node: env.connection.node,
      networkNameOrChainId: env.connection.networkNameOrChainId,
    },
  }).unwrap();

  const adjustedSignature = adjustVInSignature(
    "eth_sign",
    signature,
    args.hash,
    signer
  );

  return { signer: signer, data: adjustedSignature };
}
export function getTransactionHash(
  args: Args_getTransactionHash,
  env: Env
): string {
  const contractArgs = getTransactionHashArgs(args.tx);

  const res = Ethers_Module.callContractView({
    address: env.safeAddress,
    method:
      "function getTransactionHash(address to, uint256 value, bytes data, uint8 operation, uint256 safeTxGas, uint256 baseGas, uint256 gasPrice, address gasToken, address refundReceiver, uint256 _nonce) public view returns (bytes32)",
    args: contractArgs,
    connection: env.connection,
  }).unwrap();

  return res;
}

export function getTransactionHashArgs(tx: SafeTransactionData): string[] {
  return [
    tx.to,
    tx.value.toString(),
    tx.data,
    tx.operation!.unwrap().toString(),
    tx.safeTxGas!.toString(),
    tx.baseGas!.toString(),
    tx.gasPrice!.toString(),
    tx.gasToken!,
    tx.refundReceiver!,
    tx.nonce!.toString(),
  ];
}

export function createTransactionFromPartial(
  transactionData: SafeTransactionDataPartial
): SafeTransactionData {
  let transaction: SafeTransactionData = {
    data: transactionData.data,
    to: transactionData.to,
    value: transactionData.value,
    baseGas: BigInt.from("0"),
    gasPrice: BigInt.from("0"),
    safeTxGas: BigInt.from("0"),
    gasToken: ZERO_ADDRESS,
    nonce: 0,
    operation: Box.from(OperationType.Call),
    refundReceiver: ZERO_ADDRESS,
  };

  // TODO: if args.tx.data is parsed as an array, create multisend tx
  // let value: Box<u32> = args.tx.value != null ? args.tx.value : <u32>0;

  // Box skips '!= null' check, and 'Box.isEmpty()' can't be used if value type is Box | null

  if (transactionData.baseGas) {
    transaction.baseGas = transactionData.baseGas!;
  }

  if (transactionData.gasPrice) {
    transaction.gasPrice = transactionData.gasPrice!;
  }

  if (transactionData.safeTxGas) {
    transaction.safeTxGas = transactionData.safeTxGas!;
  }

  if (transactionData.gasToken != null) {
    transaction.gasToken = transactionData.gasToken!;
  }

  if (transactionData.nonce) {
    transaction.nonce = transactionData.nonce!.unwrap();
  }

  if (transactionData.operation) {
    transaction.operation = transactionData.operation!; // 0 is Call, 1 is DelegateCall
  }

  if (transactionData.refundReceiver != null) {
    transaction.refundReceiver = transactionData.refundReceiver!;
  }

  return transaction;
}

export function encodeSignatures(
  signatures: Map<string, SafeSignature>
): string {
  const signers = (<Array<string>>signatures.keys()).sort();
  let staticParts = "";
  let dynamicParts = "";

  for (let i = 0; i < signers.length; i++) {
    const signerAddress = signers[i];
    const signature = signatures.get(signerAddress);
    staticParts += signature!.data.slice(2); // https://github.com/safe-global/safe-core-sdk/blob/b0a6c4b518c449fd50c9d901a5a8dd171f4b064b/packages/safe-core-sdk/src/utils/transactions/SafeTransaction.ts#L22
    dynamicParts += ""; // https://github.com/safe-global/safe-core-sdk/blob/b0a6c4b518c449fd50c9d901a5a8dd171f4b064b/packages/safe-core-sdk/src/utils/signatures/SafeSignature.ts#L33
  }

  return "0x" + staticParts + dynamicParts;
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
