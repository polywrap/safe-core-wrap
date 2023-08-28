import { encodeSignatures, getVersion } from "../contracts";

import {
  Env,
  Ethers_Module,
  SafeTransactionData,
  SafeSignature,
  Ethers_TxReceipt,
  Ethers_Connection,
  SafeTransaction,
  Ethers_TxOptions,
} from "../wrap";
import { adjustVInSignature } from "./signature";
import { generateTypedData, toJsonTypedData } from "./typedData";

export function signTypedData(
  tx: SafeTransactionData,
  env: Env
): SafeSignature {
  const safeVersion = getVersion({
    address: env.safeAddress,
    connection: env.connection,
  });

  const chainId = Ethers_Module.getChainId({
    connection: env.connection,
  }).unwrap();

  const typedData = generateTypedData(
    env.safeAddress,
    safeVersion,
    chainId,
    tx
  );
  const jsonTypedData = toJsonTypedData(typedData);

  const signature = Ethers_Module.signTypedData({
    payload: jsonTypedData,
    connection: env.connection,
  }).unwrap();

  return {
    signer: Ethers_Module.getSignerAddress({
      connection: env.connection,
    }).unwrap(),
    data: adjustVInSignature("eth_signTypedData", signature, null, null),
  };
}

export function execTransaction(
  safeAddress: string,
  safeTransaction: SafeTransaction,
  txOptions: Ethers_TxOptions | null,
  connection: Ethers_Connection | null
): Ethers_TxReceipt {
  const txData = safeTransaction.data;
  const txSignatures = safeTransaction.signatures!;

  const overrideOptions: Ethers_TxOptions = {
    gasLimit: txOptions != null ? txOptions!.gasLimit : null,
    gasPrice: txOptions != null ? txOptions!.gasPrice : null,
    value: txOptions != null ? txOptions!.value : null,
    maxFeePerGas: txOptions ? txOptions!.maxFeePerGas : null,
    maxPriorityFeePerGas: txOptions ? txOptions!.maxPriorityFeePerGas : null,
    nonce: txOptions ? txOptions!.nonce : null,
  };

  const method =
    "function execTransaction(address,uint256,bytes calldata,uint8,uint256,uint256,uint256,address,address,bytes memory)";

  const encodedSignatures = encodeSignatures(txSignatures);
  // if (!txOptions.gasLimit) {
  //   const estimationArgs = getTransactionHashArgs(txData);
  //   estimationArgs.pop();
  //   estimationArgs.push(encodedSignatures);

  //   txOptions.gasLimit = this.estimateGas({
  //     address: args.safeAddress,
  //     method,
  //     args: estimationArgs,
  //     connection: args.connection,
  //   });
  // }

  return Ethers_Module.callContractMethodAndWait({
    address: safeAddress,
    method,
    args: [
      txData.to,
      txData.value.toString(),
      txData.data,
      txData.operation!.unwrap().toString(),
      txData.safeTxGas!.toString(),
      txData.baseGas!.toString(),
      txData.gasPrice!.toString(),
      txData.gasToken!,
      txData.refundReceiver!,
      encodedSignatures,
    ],
    options: overrideOptions,
    connection: connection,
  }).unwrap();
}
