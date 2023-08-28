import { BigInt } from "@polywrap/wasm-as";
import {
  Args_createProxy,
  Args_proxyCreationCode,
  DeploymentInput,
  Ethers_Connection,
  Ethers_Log,
  Ethers_Module,
  Ethers_TxOptions,
} from "../wrap";
import {
  generateSalt,
  getInitCode,
  isContractDeployed,
  prepareSafeDeployPayload,
} from "./utils";

export class SafeFactory {
  constructor(public deploymentInput: DeploymentInput) {}

  public static createProxy(args: Args_createProxy): string {
    const tx = Ethers_Module.callContractMethodAndWait({
      address: args.address,
      method: "function createProxyWithNonce(address,bytes memory,uint256)",
      args: [
        args.safeMasterCopyAddress,
        args.initializer,
        args.saltNonce.toString(),
      ],
      connection: args.connection,
      options: null,
    }).unwrap();

    // ProxyCreation(address)
    const proxyCreation_1_2_0 =
      "0xa38789425dbeee0239e16ff2d2567e31720127fbc6430758c1a4efc6aef29f80";
    // ProxyCreation(address,address)
    const proxyCreation_1_3_0 =
      "0x4f51faf6c4561ff95f067657e43439f0f856d97c04d9ec9070a6199ad418e235";
    const index = tx.logs.findIndex(
      (log: Ethers_Log) =>
        log.topics[0] == proxyCreation_1_2_0 ||
        log.topics[0] == proxyCreation_1_3_0
    );

    if (index == -1) {
      throw new Error(
        "Couldn't fetch address from event logs from transaction " +
          tx.transactionHash
      );
    }

    const address = "0x" + tx.logs[index].data.slice(32, 72);

    return address;
  }

  public static proxyCreationCode(args: Args_proxyCreationCode): string {
    return Ethers_Module.callContractView({
      address: args.address,
      method: "function proxyCreationCode() public pure returns (bytes memory)",
      args: [],
      connection: args.connection,
    }).unwrap();
  }

  public deploySafe(txOptions: Ethers_TxOptions | null): string {
    const payload = prepareSafeDeployPayload(
      this.deploymentInput.safeAccountConfig,
      this.deploymentInput.safeDeploymentConfig,
      this.deploymentInput.customContractAddresses,
      this.deploymentInput.connection
    );

    if (txOptions != null) {
      txOptions = {
        value: txOptions.value,
        gasLimit: null,
        gasPrice: null,
        maxFeePerGas: null,
        maxPriorityFeePerGas: null,
        nonce: null,
      };
    }

    let connection: Ethers_Connection | null = null;
    if (this.deploymentInput.connection != null) {
      connection = {
        node: this.deploymentInput.connection!.node,
        networkNameOrChainId:
          this.deploymentInput.connection!.networkNameOrChainId,
      };
    }
    const safeAddress = SafeFactory.createProxy({
      safeMasterCopyAddress: payload.safeContractAddress,
      address: payload.safeFactoryContractAddress,
      connection,
      initializer: payload.initializer,
      saltNonce: <u32>BigInt.from(payload.saltNonce).toUInt64(),
      txOptions,
    });

    const contractDeployed = isContractDeployed(
      safeAddress,
      this.deploymentInput.connection
    );
    if (!contractDeployed) {
      throw new Error(
        "SafeProxy contract is not deployed on the current network"
      );
    }

    return safeAddress;
  }

  public predictSafeAddress(): string {
    const payload = prepareSafeDeployPayload(
      this.deploymentInput.safeAccountConfig,
      this.deploymentInput.safeDeploymentConfig,
      this.deploymentInput.customContractAddresses,
      this.deploymentInput.connection
    );

    const salt = generateSalt(payload.saltNonce, payload.initializer);
    if (salt.isErr) {
      throw salt.err().unwrap();
    }

    let connection: Ethers_Connection | null = null;
    if (this.deploymentInput.connection != null) {
      connection = {
        node: this.deploymentInput.connection!.node,
        networkNameOrChainId:
          this.deploymentInput.connection!.networkNameOrChainId,
      };
    }
    const initCode = getInitCode(
      payload.safeFactoryContractAddress,
      payload.safeContractAddress,
      connection
    );
    if (initCode.isErr) {
      throw initCode.err().unwrap();
    }

    let derivedAddress = Ethers_Module.generateCreate2Address({
      address: payload.safeFactoryContractAddress,
      salt: salt.unwrap(),
      initCode: initCode.unwrap(),
    });

    if (derivedAddress.isErr) {
      throw derivedAddress.err().unwrap();
    }

    return derivedAddress.unwrap();
  }

  public encodeDeploySafe(): string {
    const payload = prepareSafeDeployPayload(
      this.deploymentInput.safeAccountConfig,
      this.deploymentInput.safeDeploymentConfig,
      this.deploymentInput.customContractAddresses,
      this.deploymentInput.connection
    );

    return Ethers_Module.encodeFunction({
      method: "function createProxyWithNonce(address,bytes memory,uint256)",
      args: [
        payload.safeContractAddress,
        payload.initializer,
        // @TODO(cbrzn): This should accept salt nonce as string
        BigInt.fromString(payload.saltNonce).toString(),
      ],
    }).unwrap();
  }
}
