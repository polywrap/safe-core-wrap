import { BigInt, Box } from "@polywrap/wasm-as";
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
  Args_signTransactionHash,
  ContractNetworksConfig,
  Env,
  Ethers_Module,
  Ethers_TxReceipt,
  ModuleBase,
  SafeSignature,
  SafeTransaction,
  Ethers_TxOptions,
  SafeTransactionData,
  OperationType,
  SafeTransactionDataPartial,
} from "./wrap";
import { SafeFactory } from "./factory";
import {
  createTransactionFromPartial,
  encodeSignatures,
  getSafeContractNetworks,
  getTransactionHash as getTransactionHashHelper,
  signTransactionHash as signTransactionHashHelper,
} from "./contracts";
import { encodeSetupCallData, isContractDeployed } from "./factory/utils";
import {
  encodeMultiSendData as encodeMultiSendDataHelper,
  execTransaction,
  signTypedData,
} from "./utils/transaction";
import * as ownerManager from "./managers/owner";
import * as contractManager from "./managers/contracts";
import * as moduleManager from "./managers/module";
import {
  adjustVInSignature,
  generatePreValidatedSignature,
} from "./utils/signature";
import { toTransaction, toTxReceipt } from "./utils/mappings";
import { generateTypedData, toJsonTypedData } from "./utils/typedData";

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
    return getSafeContractNetworks(args);
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

    const addressIsOwner = ownerManager.isOwner({
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
    if (args.txs.length == 0) {
      throw new Error("Invalid empty array of transactions");
    }

    if (args.txs.length == 1) {
      return this.createTransaction({ tx: args.txs[0] });
    }

    const multiSendData = encodeMultiSendDataHelper(args.txs);

    const data = Ethers_Module.encodeFunction({
      method: "function multiSend(bytes transactions)",
      args: [multiSendData],
    }).unwrap();

    const transactionData = createTransactionFromPartial({
      data: "",
      to: "",
      value: BigInt.from(""),
    } as SafeTransactionDataPartial);

    let multiSendAddress: string = "";

    if (args.customMultiSendContractAddress != null) {
      multiSendAddress = args.customMultiSendContractAddress!;
    } else {
      const chainId = Ethers_Module.getChainId({
        connection: env.connection,
      }).unwrap();
      const isL1Safe = true; // TODO figure out how get it from safe
      const version = this.getVersion({
        address: env.safeAddress,
        connection: env.connection,
      });
      const contractNetworks = this.getSafeContractNetworks({
        chainId,
        isL1Safe: Box.from(isL1Safe),
        version: version,
        filter: {
          safeMasterCopyAddress: false,
          safeProxyFactoryAddress: false,
          multiSendAddress: true,
          multiSendCallOnlyAddress: true,
          fallbackHandlerAddress: true,
        },
      });

      if (args.onlyCalls) {
        multiSendAddress = contractNetworks.multiSendCallOnlyAddress!;
      } else {
        multiSendAddress = contractNetworks.multiSendAddress!;
      }
    }

    const multiSendTransaction: SafeTransactionData = {
      to: multiSendAddress,
      value: BigInt.from("0"),
      data: data,
      operation: Box.from(OperationType.DelegateCall), // OperationType.DelegateCall,
      baseGas:
        args.options != null && args.options!.baseGas
          ? args.options!.baseGas!
          : transactionData.baseGas,
      gasPrice:
        args.options != null && args.options!.gasPrice
          ? args.options!.gasPrice!
          : transactionData.gasPrice,
      gasToken:
        args.options != null && args.options!.gasToken
          ? args.options!.gasToken!
          : transactionData.gasToken,
      nonce:
        args.options != null && args.options!.nonce
          ? args.options!.nonce!.unwrap()
          : transactionData.nonce,
      refundReceiver:
        args.options != null && args.options!.refundReceiver
          ? args.options!.refundReceiver!
          : transactionData.refundReceiver,
      safeTxGas:
        args.options != null && args.options!.safeTxGas
          ? args.options!.safeTxGas!
          : transactionData.safeTxGas,
    };

    return {
      data: multiSendTransaction,
      signatures: new Map<string, SafeSignature>(),
    };
  }
  getTransactionHash(args: Args_getTransactionHash, env: Env): string {
    return getTransactionHashHelper(args, env);
  }
  signTransactionHash(args: Args_signTransactionHash, env: Env): SafeSignature {
    return signTransactionHashHelper(args, env);
  }
  approveTransactionHash(
    args: Args_approveTransactionHash,
    env: Env
  ): Ethers_TxReceipt {
    const signerAddress = Ethers_Module.getSignerAddress({
      connection: env.connection,
    }).unwrap();

    const addressIsOwner = ownerManager.isOwner({
      ownerAddress: signerAddress,
      safeAddress: env.safeAddress,
      connection: env.connection,
    });

    if (!addressIsOwner) {
      throw new Error("Transaction hashes can only be approved by Safe owners");
    }

    const options: Ethers_TxOptions = {
      gasPrice: null,
      nonce: null,
      value: null,
      maxFeePerGas: null,
      maxPriorityFeePerGas: null,
      gasLimit: null,
    };

    if (args.options) {
      if (args.options!.gasPrice) {
        options.gasPrice = args.options!.gasPrice;
      }

      if (args.options!.gasLimit) {
        options.gasLimit = args.options!.gasLimit;
      }
    }

    const response = Ethers_Module.callContractMethodAndWait({
      method: "function approveHash(bytes32 hashToApprove)",
      address: env.safeAddress,
      args: [args.hash],
      connection: env.connection,
      options,
    }).unwrap();

    return response;
  }
  approvedHashes(args: Args_approvedHashes): BigInt {
    const result = Ethers_Module.callContractView({
      address: args.address,
      method:
        "function approvedHashes(address owner, bytes32 hash) public view returns (uint256)",
      args: [args.ownerAddress, args.hash],
      connection: args.connection,
    }).unwrap();
    return BigInt.from(result);
  }
  approveHash(args: Args_approveHash): Ethers_TxReceipt {
    const signerAddress = Ethers_Module.getSignerAddress({
      connection: args.connection,
    }).unwrap();

    const addressIsOwner = this.isOwner({
      safeAddress: args.safeAddress,
      ownerAddress: signerAddress,
      connection: args.connection,
    });

    if (!addressIsOwner) {
      throw new Error("Transaction hashes can only be approved by Safe owners");
    }

    if (
      args.options != null &&
      args.options!.gasPrice &&
      args.options!.gasLimit
    ) {
      throw new Error(
        "Cannot specify gas and gasLimit together in transaction options"
      );
    }

    const nonce = args.options ? args.options!.nonce : null;

    const response = Ethers_Module.callContractMethodAndWait({
      method: "function approveHash(bytes32 hashToApprove) external",
      address: args.safeAddress,
      args: [args.hash],
      connection: args.connection,
      options: {
        gasLimit: args.options ? args.options!.gasLimit : null,
        gasPrice: args.options ? args.options!.gasPrice : null,
        value: null,
        maxFeePerGas: args.options ? args.options!.maxFeePerGas : null,
        maxPriorityFeePerGas: args.options
          ? BigInt.from(args.options!.maxPriorityFeePerGas)
          : null,
        nonce: nonce ? Box.from(nonce.toUInt32()) : null,
      },
    }).unwrap();

    return response;
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

    const owners = ownerManager.getOwners({
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

    const threshold = ownerManager.getThreshold({
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

      if (args.options!.maxFeePerGas) {
        txOptions.maxFeePerGas = args.options!.maxFeePerGas;
      }

      if (args.options!.maxPriorityFeePerGas) {
        txOptions.maxPriorityFeePerGas = BigInt.from(
          args.options!.maxPriorityFeePerGas!
        );
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
    const owners = ownerManager.getOwners({
      safeAddress: env.safeAddress,
      connection: env.connection,
    });
    const ownersWhoApproved: string[] = [];

    for (let i = 0; i < owners.length; i++) {
      const owner = owners[i];
      const approved = ownerManager.approvedHashes({
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
    const signerAddress = Ethers_Module.getSignerAddress({
      connection: {
        node: args.connection.node,
        networkNameOrChainId: args.connection.networkNameOrChainId,
      },
    }).unwrap();

    let signatures = args.tx.signatures;

    //If signature of current signer is already present - return transaction
    if (signatures != null) {
      if (signatures.has(signerAddress)) {
        return args.tx;
      }
    }

    const chainId = Ethers_Module.getChainId({
      connection: args.connection,
    }).unwrap();

    //If no signatures - create signatures map
    if (signatures == null) {
      signatures = new Map<string, SafeSignature>();
    }

    const typedData = generateTypedData(
      args.safeAddress,
      "1.3.0",
      chainId,
      args.tx.data
    );
    const payload = toJsonTypedData(typedData);

    const signature = Ethers_Module.signTypedData({
      payload,
      connection: args.connection,
    }).unwrap();

    signatures.set(signerAddress, {
      signer: signerAddress,
      data: adjustVInSignature("eth_signTypedData", signature, null, null),
    });

    //Add signature of current signer
    args.tx.signatures = signatures;

    return args.tx;
  }
  getVersion(args: Args_getVersion): string {
    return contractManager.getVersion(args);
  }
  getOwners(args: Args_getOwners): string[] {
    return ownerManager.getOwners(args);
  }
  getThreshold(args: Args_getThreshold): u32 {
    return ownerManager.getThreshold(args);
  }
  isOwner(args: Args_isOwner): boolean {
    return ownerManager.isOwner(args);
  }
  getModules(args: Args_getModules, env: Env): string[] {
    return moduleManager.getModules(args, env);
  }
  isModuleEnabled(args: Args_isModuleEnabled, env: Env): boolean {
    return moduleManager.isModuleEnabled(args, env);
  }

  // Encode utilities
  encodeExecTransaction(args: Args_encodeExecTransaction): string {
    const txData = args.safeTransaction.data;
    const txSignatures = args.safeTransaction.signatures!;
    const method =
      "function execTransaction(address,uint256,bytes calldata,uint8,uint256,uint256,uint256,address,address,bytes memory)";
    const encodedSignatures = encodeSignatures(txSignatures);
    return Ethers_Module.encodeFunction({
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
    }).unwrap();
  }
  encodeEnableModuleData(args: Args_encodeEnableModuleData, env: Env): string {
    return moduleManager.encodeEnableModuleData(args, env);
  }
  encodeDisableModuleData(
    args: Args_encodeDisableModuleData,
    env: Env
  ): string {
    return moduleManager.encodeDisableModuleData(args, env);
  }
  encodeMultiSendData(args: Args_encodeMultiSendData): string {
    const multiSendData = encodeMultiSendDataHelper(args.txs);

    const encodedMultisend = Ethers_Module.encodeFunction({
      method: "function multiSend(bytes transactions)",
      args: [multiSendData],
    }).unwrap();
    return encodedMultisend;
  }

  encodeAddOwnerWithThresholdData(
    args: Args_encodeAddOwnerWithThresholdData,
    env: Env
  ): string {
    return ownerManager.encodeAddOwnerWithThresholdData(args, env);
  }
  encodeRemoveOwnerData(args: Args_encodeRemoveOwnerData, env: Env): string {
    return ownerManager.encodeRemoveOwnerData(args, env);
  }
  encodeSwapOwnerData(args: Args_encodeSwapOwnerData, env: Env): string {
    return ownerManager.encodeSwapOwnerData(args, env);
  }
  encodeChangeThresholdData(
    args: Args_encodeChangeThresholdData,
    env: Env
  ): string {
    return ownerManager.encodeChangeThresholdData(args, env);
  }
}
