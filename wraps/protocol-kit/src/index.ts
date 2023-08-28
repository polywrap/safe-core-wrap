import { BigInt } from "@polywrap/wasm-as";
import {
  Args_addSignature,
  Args_approveHash,
  Args_approveTransactionHash,
  Args_approvedHashes,
  Args_createMultiSendTransaction,
  Args_createProxy,
  Args_createTransaction,
  Args_deploySafe,
  Args_encodeAddOwnerWithThresholdData,
  Args_encodeChangeThresholdData,
  Args_encodeDeploySafe,
  Args_encodeDisableModuleData,
  Args_encodeEnableModuleData,
  Args_encodeExecTransaction,
  Args_encodeMultiSendData,
  Args_encodeRemoveOwnerData,
  Args_encodeSwapOwnerData,
  Args_executeTransaction,
  Args_getModules,
  Args_getNonce,
  Args_getOwners,
  Args_getOwnersWhoApprovedTx,
  Args_getSafeContractNetworks,
  Args_getSafeInitializer,
  Args_getSignature,
  Args_getThreshold,
  Args_getTransactionHash,
  Args_getVersion,
  Args_isModuleEnabled,
  Args_isOwner,
  Args_predictSafeAddress,
  Args_proxyCreationCode,
  Args_safeIsDeployed,
  Args_signTransaction,
  Args_signTransactionHash,
  ContractNetworksConfig,
  Env,
  Ethers_Module,
  Ethers_TxReceipt,
  ModuleBase,
  SafeSignature,
  SafeTransaction,
  Ethers_TxOptions,
} from "./wrap";
import { SafeFactory } from "./factory";
import * as ContractHelpers from "./contracts";
import { encodeSetupCallData, isContractDeployed } from "./factory/utils";
import { createTransactionFromPartial } from "./contracts/utils";
import { execTransaction, signTypedData } from "./utils/transaction";
import { generatePreValidatedSignature } from "./utils/signature";
import { toTransaction, toTxReceipt } from "./utils/mappings";
import { approvedHashes } from "./managers/owner";

export class Module extends ModuleBase {
  createProxy(args: Args_createProxy): string {
    return SafeFactory.createProxy(args);
  }
  proxyCreationCode(args: Args_proxyCreationCode): string {
    return SafeFactory.proxyCreationCode(args);
  }
  getSafeContractNetworks(
    args: Args_getSafeContractNetworks
  ): ContractNetworksConfig {
    return ContractHelpers.getSafeContractNetworks(args);
  }
  getNonce(args: Args_getNonce): BigInt {
    const nonce = Ethers_Module.callContractView({
      address: args.address,
      method: "function nonce() public view returns (uint256)",
      args: [],
      connection: args.connection,
    }).unwrap();

    return BigInt.from(nonce);
  }

  // Factory methods
  deploySafe(args: Args_deploySafe): string {
    const factory = new SafeFactory(args.input);
    return factory.deploySafe(args.txOptions);
  }
  predictSafeAddress(args: Args_predictSafeAddress): string {
    const factory = new SafeFactory(args.input);
    return factory.predictSafeAddress();
  }
  encodeDeploySafe(args: Args_encodeDeploySafe): string {
    const factory = new SafeFactory(args.input);
    return factory.encodeDeploySafe();
  }
  safeIsDeployed(args: Args_safeIsDeployed): boolean {
    return isContractDeployed(args.safeAddress, args.connection);
  }
  getSafeInitializer(args: Args_getSafeInitializer): string {
    return encodeSetupCallData(args.config);
  }

  // Transaction methods
  createTransaction(args: Args_createTransaction): SafeTransaction {
    const transactionData = createTransactionFromPartial(args.tx);

    return {
      data: transactionData,
      signatures: new Map<string, SafeSignature>(),
    };
  }
  addSignature(args: Args_addSignature, env: Env): SafeTransaction {
    const signerAddress = Ethers_Module.getSignerAddress({
      connection: {
        node: env.connection.node,
        networkNameOrChainId: env.connection.networkNameOrChainId,
      },
    }).unwrap();

    const addressIsOwner = ContractHelpers.isOwner({
      ownerAddress: signerAddress,
      safeAddress: env.safeAddress,
      connection: env.connection,
    });

    if (addressIsOwner == false) {
      throw new Error("Transactions can only be signed by Safe owners");
    }

    let signatures = args.tx.signatures;

    //If signature of current signer is already present - return transaction
    if (signatures != null) {
      if (signatures.has(signerAddress)) {
        return args.tx;
      }
    }

    //If no signatures - create signatures map
    if (signatures == null) {
      signatures = new Map<string, SafeSignature>();
    }
    if (
      args.signingMethod != null &&
      args.signingMethod! == "eth_signTypedData"
    ) {
      const signature = signTypedData(args.tx.data, env);
      signatures.set(signerAddress, signature);
    } else {
      const transactionHash = this.getTransactionHash(
        { tx: args.tx.data },
        env
      );
      const signature = this.signTransactionHash(
        { hash: transactionHash },
        env
      );
      signatures.set(signerAddress, signature);
    }
    //Add signature of current signer
    args.tx.signatures = signatures;

    return args.tx;
  }
  createMultiSendTransaction(
    args: Args_createMultiSendTransaction,
    env: Env
  ): SafeTransaction {
    throw new Error("Method not implemented.");
  }
  getTransactionHash(args: Args_getTransactionHash, env: Env): string {
    return ContractHelpers.getTransactionHash(args, env);
  }
  signTransactionHash(args: Args_signTransactionHash, env: Env): SafeSignature {
    return ContractHelpers.signTransactionHash(args, env);
  }
  signTransaction(args: Args_signTransaction, env: Env): SafeSignature {
    throw new Error("Method not implemented.");
  }
  approveTransactionHash(
    args: Args_approveTransactionHash,
    env: Env
  ): Ethers_TxReceipt {
    throw new Error("Method not implemented.");
  }
  approvedHashes(args: Args_approvedHashes): BigInt {
    throw new Error("Method not implemented.");
  }
  approveHash(args: Args_approveHash): Ethers_TxReceipt {
    throw new Error("Method not implemented.");
  }
  executeTransaction(
    args: Args_executeTransaction,
    env: Env
  ): Ethers_TxReceipt {
    const transaction = args.tx;

    const signedSafeTransaction = args.tx;

    for (let i = 0; i < transaction.signatures!.keys().length; i++) {
      const key = transaction.signatures!.keys()[i];

      const signature = transaction.signatures!.get(key);

      signedSafeTransaction.signatures!.set(
        signature.signer.toLowerCase(),
        signature
      );
    }

    const txHash = this.getTransactionHash(
      { tx: signedSafeTransaction.data },
      env
    );

    const ownersWhoApprovedTx = this.getOwnersWhoApprovedTx(
      { hash: txHash },
      env
    );
    for (let i = 0; i < ownersWhoApprovedTx.length; i++) {
      const owner = ownersWhoApprovedTx[i];
      signedSafeTransaction.signatures!.set(
        owner.toLowerCase(),
        generatePreValidatedSignature(owner)
      );
    }

    const owners = ContractHelpers.getOwners({
      safeAddress: env.safeAddress,
      connection: env.connection,
    });

    const signerAddress = Ethers_Module.getSignerAddress({
      connection: env.connection,
    }).unwrap();

    if (owners.includes(signerAddress)) {
      signedSafeTransaction.signatures!.set(
        signerAddress.toLowerCase(),
        generatePreValidatedSignature(signerAddress)
      );
    }

    const threshold = ContractHelpers.getThreshold({
      safeAddress: env.safeAddress,
      connection: env.connection,
    });

    if (threshold > <u32>signedSafeTransaction.signatures!.size) {
      const signaturesMissing =
        threshold - signedSafeTransaction.signatures!.size;
      throw new Error(
        `There ${
          signaturesMissing > 1 ? "are" : "is"
        } ${signaturesMissing} signature${
          signaturesMissing > 1 ? "s" : ""
        } missing`
      );
    }

    const value = BigInt.from(signedSafeTransaction.data.value);

    if (!value.isZero()) {
      const balance = Ethers_Module.getBalance({
        address: env.safeAddress,
        blockTag: null,
        connection: env.connection,
      }).unwrap();
      if (value.gt(BigInt.from(balance))) {
        throw new Error("Not enough Ether funds");
      }
    }

    const txOptions: Ethers_TxOptions = {
      gasLimit: null,
      gasPrice: null,
      value: null,
      maxFeePerGas: null,
      maxPriorityFeePerGas: null,
      nonce: null,
    };

    if (args.options) {
      if (args.options!.gas && args.options!.gasLimit) {
        throw new Error(
          "Cannot specify gas and gasLimit together in transaction options"
        );
      }

      if (args.options!.gasPrice) {
        txOptions.gasPrice = args.options!.gasPrice;
      }

      if (args.options!.gasLimit) {
        txOptions.gasLimit = args.options!.gasLimit;
      }
    }

    const txReceipt = execTransaction(
      env.safeAddress,
      toTransaction(signedSafeTransaction),
      txOptions,
      {
        networkNameOrChainId: env.connection.networkNameOrChainId,
        node: env.connection.node,
      }
    );

    return toTxReceipt(txReceipt);
  }

  // Manager helper methods
  getOwnersWhoApprovedTx(
    args: Args_getOwnersWhoApprovedTx,
    env: Env
  ): string[] {
    const owners = ContractHelpers.getOwners({
      safeAddress: env.safeAddress,
      connection: env.connection,
    });
    const ownersWhoApproved: string[] = [];

    for (let i = 0; i < owners.length; i++) {
      const owner = owners[i];
      const approved = approvedHashes({
        hash: args.hash,
        ownerAddress: owner,
        address: env.safeAddress,
        connection: env.connection,
      });
      if (approved.gt(0)) {
        ownersWhoApproved.push(owner);
      }
    }
    return ownersWhoApproved;
  }
  getSignature(args: Args_getSignature): SafeTransaction {
    throw new Error("Method not implemented.");
  }
  getVersion(args: Args_getVersion): string {
    return ContractHelpers.getVersion(args);
  }
  getOwners(args: Args_getOwners): string[] {
    return ContractHelpers.getOwners(args);
  }
  getThreshold(args: Args_getThreshold): u32 {
    return ContractHelpers.getThreshold(args);
  }
  isOwner(args: Args_isOwner): boolean {
    return ContractHelpers.isOwner(args);
  }
  getModules(args: Args_getModules, env: Env): string[] {
    throw new Error("Method not implemented.");
  }
  isModuleEnabled(args: Args_isModuleEnabled, env: Env): boolean {
    throw new Error("Method not implemented.");
  }

  // Encode utilities
  encodeExecTransaction(args: Args_encodeExecTransaction): string {
    throw new Error("Method not implemented.");
  }
  encodeEnableModuleData(args: Args_encodeEnableModuleData, env: Env): string {
    throw new Error("Method not implemented.");
  }
  encodeDisableModuleData(
    args: Args_encodeDisableModuleData,
    env: Env
  ): string {
    throw new Error("Method not implemented.");
  }
  encodeMultiSendData(args: Args_encodeMultiSendData): string {
    throw new Error("Method not implemented.");
  }
  encodeAddOwnerWithThresholdData(
    args: Args_encodeAddOwnerWithThresholdData,
    env: Env
  ): string {
    throw new Error("Method not implemented.");
  }
  encodeRemoveOwnerData(args: Args_encodeRemoveOwnerData, env: Env): string {
    throw new Error("Method not implemented.");
  }
  encodeSwapOwnerData(args: Args_encodeSwapOwnerData, env: Env): string {
    throw new Error("Method not implemented.");
  }
  encodeChangeThresholdData(
    args: Args_encodeChangeThresholdData,
    env: Env
  ): string {
    throw new Error("Method not implemented.");
  }
}
