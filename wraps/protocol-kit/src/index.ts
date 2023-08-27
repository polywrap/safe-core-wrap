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
  Args_execTransaction,
  Args_executeTransaction,
  Args_getAddress,
  Args_getContractVersion,
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
  Ethers_TxReceipt,
  ModuleBase,
  SafeSignature,
  SafeTransaction,
} from "./wrap";
import { SafeFactory } from "./factory";

export class Module extends ModuleBase {
  createProxy(args: Args_createProxy): string {
    return SafeFactory.createProxy(args);
  }
  proxyCreationCode(args: Args_proxyCreationCode): string {
    return SafeFactory.proxyCreationCode(args);
  }
  deploySafe(args: Args_deploySafe): string {
    const factory = new SafeFactory(args.input);
    return factory.deploySafe(args.txOptions);
  }
  predictSafeAddress(args: Args_predictSafeAddress): string {
    const factory = new SafeFactory(args.input);
    return factory.predictSafeAddress();
  }
  getVersion(args: Args_getVersion): string {
    throw new Error("Method not implemented.");
  }
  approvedHashes(args: Args_approvedHashes): BigInt {
    throw new Error("Method not implemented.");
  }
  approveHash(args: Args_approveHash): Ethers_TxReceipt {
    throw new Error("Method not implemented.");
  }
  execTransaction(args: Args_execTransaction): Ethers_TxReceipt {
    throw new Error("Method not implemented.");
  }
  getSafeContractNetworks(
    args: Args_getSafeContractNetworks
  ): ContractNetworksConfig {
    throw new Error("Method not implemented.");
  }
  encodeExecTransaction(args: Args_encodeExecTransaction): string {
    throw new Error("Method not implemented.");
  }
  safeIsDeployed(args: Args_safeIsDeployed): bool {
    throw new Error("Method not implemented.");
  }
  encodeDeploySafe(args: Args_encodeDeploySafe): string {
    throw new Error("Method not implemented.");
  }
  getSafeInitializer(args: Args_getSafeInitializer): string {
    throw new Error("Method not implemented.");
  }
  createTransaction(args: Args_createTransaction, env: Env): SafeTransaction {
    throw new Error("Method not implemented.");
  }
  createMultiSendTransaction(
    args: Args_createMultiSendTransaction,
    env: Env
  ): SafeTransaction {
    throw new Error("Method not implemented.");
  }
  addSignature(args: Args_addSignature, env: Env): SafeTransaction {
    throw new Error("Method not implemented.");
  }
  getTransactionHash(args: Args_getTransactionHash, env: Env): string | null {
    throw new Error("Method not implemented.");
  }
  signTransactionHash(args: Args_signTransactionHash, env: Env): SafeSignature {
    throw new Error("Method not implemented.");
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
  executeTransaction(
    args: Args_executeTransaction,
    env: Env
  ): Ethers_TxReceipt {
    throw new Error("Method not implemented.");
  }
  getOwnersWhoApprovedTx(
    args: Args_getOwnersWhoApprovedTx,
    env: Env
  ): string[] {
    throw new Error("Method not implemented.");
  }
  getSignature(args: Args_getSignature): SafeTransaction {
    throw new Error("Method not implemented.");
  }
  getOwners(args: Args_getOwners, env: Env): string[] {
    throw new Error("Method not implemented.");
  }
  getThreshold(args: Args_getThreshold, env: Env): u32 {
    throw new Error("Method not implemented.");
  }
  isOwner(args: Args_isOwner, env: Env): bool {
    throw new Error("Method not implemented.");
  }
  getModules(args: Args_getModules, env: Env): string[] {
    throw new Error("Method not implemented.");
  }
  isModuleEnabled(args: Args_isModuleEnabled, env: Env): bool {
    throw new Error("Method not implemented.");
  }
  getAddress(args: Args_getAddress, env: Env): string {
    throw new Error("Method not implemented.");
  }
  getContractVersion(args: Args_getContractVersion, env: Env): string {
    throw new Error("Method not implemented.");
  }
  getNonce(args: Args_getNonce, env: Env): BigInt {
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
