#![allow(unused_imports)]
#![allow(non_camel_case_types)]

// NOTE: This is an auto-generated file.
//       All modifications will be overwritten.
use polywrap::*;
use std::result::Result;
use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};
use std::sync::Arc;

pub type BigInt = String;

// Env START //

// Env END //

// Objects START //

// Objects END //

// Enums START //

// Enums END //

// Imported objects START //

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountAbstractionDeploymentParameters {
    #[serde(rename = "saltNonce")]
    pub salt_nonce: Option<String>,
    #[serde(rename = "customContractAddresses")]
    pub custom_contract_addresses: Option<AccountAbstractionSafeCustomContract>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountAbstractionSafeCustomContract {
    #[serde(rename = "proxyFactoryContract")]
    pub proxy_factory_contract: Option<String>,
    #[serde(rename = "safeFactoryContract")]
    pub safe_factory_contract: Option<String>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountAbstractionMetaTransactionData {
    pub to: String,
    pub value: BigInt,
    pub data: String,
    pub operation: Option<AccountAbstractionOperationType>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountAbstractionMetaTransactionOptions {
    #[serde(rename = "gasLimit")]
    pub gas_limit: BigInt,
    #[serde(rename = "gasToken")]
    pub gas_token: Option<String>,
    #[serde(rename = "isSponsored")]
    pub is_sponsored: Option<bool>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountAbstractionRelayTransaction {
    pub target: String,
    #[serde(rename = "encodedTransaction")]
    pub encoded_transaction: String,
    #[serde(rename = "chainId")]
    pub chain_id: i32,
    pub options: AccountAbstractionMetaTransactionOptions,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountAbstractionEthersConnection {
    pub node: Option<String>,
    #[serde(rename = "networkNameOrChainId")]
    pub network_name_or_chain_id: Option<String>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersConnection {
    pub node: Option<String>,
    #[serde(rename = "networkNameOrChainId")]
    pub network_name_or_chain_id: Option<String>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersTxRequest {
    pub to: Option<String>,
    pub from: Option<String>,
    pub data: Option<String>,
    #[serde(rename = "type")]
    pub _type: Option<u32>,
    #[serde(rename = "chainId")]
    pub chain_id: Option<BigInt>,
    #[serde(rename = "accessList")]
    pub access_list: Option<Vec<EthersAccessItem>>,
    #[serde(rename = "gasLimit")]
    pub gas_limit: Option<BigInt>,
    #[serde(rename = "maxFeePerGas")]
    pub max_fee_per_gas: Option<BigInt>,
    #[serde(rename = "maxPriorityFeePerGas")]
    pub max_priority_fee_per_gas: Option<BigInt>,
    #[serde(rename = "gasPrice")]
    pub gas_price: Option<BigInt>,
    pub value: Option<BigInt>,
    pub nonce: Option<u32>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersAccessItem {
    pub address: String,
    #[serde(rename = "storageKeys")]
    pub storage_keys: Vec<String>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersTxResponse {
    pub hash: String,
    pub to: Option<String>,
    pub from: String,
    pub nonce: u32,
    #[serde(rename = "gasLimit")]
    pub gas_limit: BigInt,
    #[serde(rename = "maxFeePerGas")]
    pub max_fee_per_gas: Option<BigInt>,
    #[serde(rename = "maxPriorityFeePerGas")]
    pub max_priority_fee_per_gas: Option<BigInt>,
    #[serde(rename = "gasPrice")]
    pub gas_price: Option<BigInt>,
    pub value: BigInt,
    #[serde(rename = "chainId")]
    pub chain_id: BigInt,
    #[serde(rename = "blockNumber")]
    pub block_number: Option<BigInt>,
    #[serde(rename = "blockHash")]
    pub block_hash: Option<String>,
    pub timestamp: Option<u32>,
    pub r: Option<String>,
    pub s: Option<String>,
    pub v: Option<u32>,
    #[serde(rename = "type")]
    pub _type: Option<u32>,
    #[serde(rename = "accessList")]
    pub access_list: Option<Vec<EthersAccessItem>>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersLog {
    #[serde(rename = "blockNumber")]
    pub block_number: BigInt,
    #[serde(rename = "blockHash")]
    pub block_hash: String,
    #[serde(rename = "transactionIndex")]
    pub transaction_index: u32,
    pub removed: bool,
    pub address: String,
    pub data: String,
    pub topics: Vec<String>,
    #[serde(rename = "transactionHash")]
    pub transaction_hash: String,
    #[serde(rename = "logIndex")]
    pub log_index: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersTxReceipt {
    pub to: String,
    pub from: String,
    #[serde(rename = "contractAddress")]
    pub contract_address: String,
    #[serde(rename = "transactionIndex")]
    pub transaction_index: u32,
    pub root: Option<String>,
    #[serde(rename = "gasUsed")]
    pub gas_used: BigInt,
    #[serde(rename = "logsBloom")]
    pub logs_bloom: String,
    #[serde(rename = "transactionHash")]
    pub transaction_hash: String,
    pub logs: Vec<EthersLog>,
    #[serde(rename = "blockNumber")]
    pub block_number: BigInt,
    #[serde(rename = "blockHash")]
    pub block_hash: String,
    pub confirmations: u32,
    #[serde(rename = "cumulativeGasUsed")]
    pub cumulative_gas_used: BigInt,
    #[serde(rename = "effectiveGasPrice")]
    pub effective_gas_price: BigInt,
    #[serde(rename = "type")]
    pub _type: u32,
    pub status: Option<u32>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersTxOptions {
    #[serde(rename = "gasLimit")]
    pub gas_limit: Option<BigInt>,
    #[serde(rename = "maxFeePerGas")]
    pub max_fee_per_gas: Option<BigInt>,
    #[serde(rename = "maxPriorityFeePerGas")]
    pub max_priority_fee_per_gas: Option<BigInt>,
    #[serde(rename = "gasPrice")]
    pub gas_price: Option<BigInt>,
    pub value: Option<BigInt>,
    pub nonce: Option<u32>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersStaticTxResult {
    pub result: String,
    pub error: bool,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersEip1559FeesEstimate {
    #[serde(rename = "maxFeePerGas")]
    pub max_fee_per_gas: BigInt,
    #[serde(rename = "maxPriorityFeePerGas")]
    pub max_priority_fee_per_gas: BigInt,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RelayerRelayTransaction {
    pub target: String,
    #[serde(rename = "encodedTransaction")]
    pub encoded_transaction: String,
    #[serde(rename = "chainId")]
    pub chain_id: i32,
    pub options: RelayerMetaTransactionOptions,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RelayerMetaTransactionOptions {
    #[serde(rename = "gasLimit")]
    pub gas_limit: BigInt,
    #[serde(rename = "gasToken")]
    pub gas_token: Option<String>,
    #[serde(rename = "isSponsored")]
    pub is_sponsored: Option<bool>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RelayerRelayResponse {
    #[serde(rename = "taskId")]
    pub task_id: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeContractNetworksConfig {
    #[serde(rename = "multiSendAddress")]
    pub multi_send_address: Option<String>,
    #[serde(rename = "multiSendCallOnlyAddress")]
    pub multi_send_call_only_address: Option<String>,
    #[serde(rename = "safeMasterCopyAddress")]
    pub safe_master_copy_address: Option<String>,
    #[serde(rename = "safeProxyFactoryAddress")]
    pub safe_proxy_factory_address: Option<String>,
    #[serde(rename = "fallbackHandlerAddress")]
    pub fallback_handler_address: Option<String>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeContractNetworksFilter {
    #[serde(rename = "multiSendAddress")]
    pub multi_send_address: bool,
    #[serde(rename = "multiSendCallOnlyAddress")]
    pub multi_send_call_only_address: bool,
    #[serde(rename = "safeMasterCopyAddress")]
    pub safe_master_copy_address: bool,
    #[serde(rename = "safeProxyFactoryAddress")]
    pub safe_proxy_factory_address: bool,
    #[serde(rename = "fallbackHandlerAddress")]
    pub fallback_handler_address: bool,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeCustomContract {
    #[serde(rename = "proxyFactoryContract")]
    pub proxy_factory_contract: Option<String>,
    #[serde(rename = "safeFactoryContract")]
    pub safe_factory_contract: Option<String>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeDeploymentPayload {
    #[serde(rename = "safeContractAddress")]
    pub safe_contract_address: String,
    #[serde(rename = "safeFactoryContractAddress")]
    pub safe_factory_contract_address: String,
    pub initializer: String,
    #[serde(rename = "saltNonce")]
    pub salt_nonce: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeSafeAccountConfig {
    pub owners: Vec<String>,
    pub threshold: u32,
    pub to: Option<String>,
    pub data: Option<String>,
    #[serde(rename = "fallbackHandler")]
    pub fallback_handler: Option<String>,
    #[serde(rename = "paymentToken")]
    pub payment_token: Option<String>,
    pub payment: Option<BigInt>,
    #[serde(rename = "paymentReceiver")]
    pub payment_receiver: Option<String>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeSafeDeploymentConfig {
    #[serde(rename = "saltNonce")]
    pub salt_nonce: String,
    #[serde(rename = "isL1Safe")]
    pub is_l1_safe: Option<bool>,
    pub version: Option<String>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeDeploymentInput {
    #[serde(rename = "safeAccountConfig")]
    pub safe_account_config: SafeSafeAccountConfig,
    #[serde(rename = "safeDeploymentConfig")]
    pub safe_deployment_config: Option<SafeSafeDeploymentConfig>,
    #[serde(rename = "customContractAddresses")]
    pub custom_contract_addresses: Option<SafeCustomContract>,
    pub connection: Option<SafeEthersConnection>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeEthersConnection {
    pub node: Option<String>,
    #[serde(rename = "networkNameOrChainId")]
    pub network_name_or_chain_id: Option<String>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeSafeSignature {
    pub signer: String,
    pub data: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeSafeTransaction {
    pub signatures: Option<BTreeMap<String, SafeSafeSignature>>,
    pub data: SafeSafeTransactionData,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeSafeTransactionData {
    #[serde(rename = "safeTxGas")]
    pub safe_tx_gas: BigInt,
    #[serde(rename = "baseGas")]
    pub base_gas: BigInt,
    #[serde(rename = "gasPrice")]
    pub gas_price: BigInt,
    #[serde(rename = "gasToken")]
    pub gas_token: String,
    #[serde(rename = "refundReceiver")]
    pub refund_receiver: String,
    pub nonce: i32,
    pub to: String,
    pub value: BigInt,
    pub data: String,
    pub operation: Option<SafeOperationType>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeMetaTransactionData {
    pub to: String,
    pub value: BigInt,
    pub data: String,
    pub operation: Option<SafeOperationType>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeTransactionOptions {
    pub from: Option<String>,
    pub gas: Option<BigInt>,
    #[serde(rename = "gasLimit")]
    pub gas_limit: Option<BigInt>,
    #[serde(rename = "gasPrice")]
    pub gas_price: Option<BigInt>,
    #[serde(rename = "maxFeePerGas")]
    pub max_fee_per_gas: Option<BigInt>,
    #[serde(rename = "maxPriorityFeePerGas")]
    pub max_priority_fee_per_gas: Option<String>,
    pub nonce: Option<BigInt>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeSafeTransactionDataPartial {
    #[serde(rename = "safeTxGas")]
    pub safe_tx_gas: Option<BigInt>,
    #[serde(rename = "baseGas")]
    pub base_gas: Option<BigInt>,
    #[serde(rename = "gasPrice")]
    pub gas_price: Option<BigInt>,
    #[serde(rename = "gasToken")]
    pub gas_token: Option<String>,
    #[serde(rename = "refundReceiver")]
    pub refund_receiver: Option<String>,
    pub nonce: Option<i32>,
    pub to: String,
    pub value: BigInt,
    pub data: String,
    pub operation: Option<SafeOperationType>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeSafeTransactionOptions {
    #[serde(rename = "safeTxGas")]
    pub safe_tx_gas: Option<BigInt>,
    #[serde(rename = "baseGas")]
    pub base_gas: Option<BigInt>,
    #[serde(rename = "gasPrice")]
    pub gas_price: Option<BigInt>,
    #[serde(rename = "gasToken")]
    pub gas_token: Option<String>,
    #[serde(rename = "refundReceiver")]
    pub refund_receiver: Option<String>,
    pub nonce: Option<i32>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeEthersTxOptions {
    #[serde(rename = "gasLimit")]
    pub gas_limit: Option<BigInt>,
    #[serde(rename = "maxFeePerGas")]
    pub max_fee_per_gas: Option<BigInt>,
    #[serde(rename = "maxPriorityFeePerGas")]
    pub max_priority_fee_per_gas: Option<BigInt>,
    #[serde(rename = "gasPrice")]
    pub gas_price: Option<BigInt>,
    pub value: Option<BigInt>,
    pub nonce: Option<u32>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeEthersTxReceipt {
    pub to: String,
    pub from: String,
    #[serde(rename = "contractAddress")]
    pub contract_address: String,
    #[serde(rename = "transactionIndex")]
    pub transaction_index: u32,
    pub root: Option<String>,
    #[serde(rename = "gasUsed")]
    pub gas_used: BigInt,
    #[serde(rename = "logsBloom")]
    pub logs_bloom: String,
    #[serde(rename = "transactionHash")]
    pub transaction_hash: String,
    pub logs: Vec<SafeEthersLog>,
    #[serde(rename = "blockNumber")]
    pub block_number: BigInt,
    #[serde(rename = "blockHash")]
    pub block_hash: String,
    pub confirmations: u32,
    #[serde(rename = "cumulativeGasUsed")]
    pub cumulative_gas_used: BigInt,
    #[serde(rename = "effectiveGasPrice")]
    pub effective_gas_price: BigInt,
    #[serde(rename = "type")]
    pub _type: u32,
    pub status: Option<u32>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeEthersLog {
    #[serde(rename = "blockNumber")]
    pub block_number: BigInt,
    #[serde(rename = "blockHash")]
    pub block_hash: String,
    #[serde(rename = "transactionIndex")]
    pub transaction_index: u32,
    pub removed: bool,
    pub address: String,
    pub data: String,
    pub topics: Vec<String>,
    #[serde(rename = "transactionHash")]
    pub transaction_hash: String,
    #[serde(rename = "logIndex")]
    pub log_index: u32,
}
// Imported objects END //

// Imported envs START //

// Imported envs END //

// Imported enums START //

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum AccountAbstractionOperationType {
    Call,
    DelegateCall,
    _MAX_
}
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum SafeOperationType {
    Call,
    DelegateCall,
    _MAX_
}
// Imported enums END //

// Imported Modules START //

// URI: "wrapscan.io/polywrap/account-abstraction-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountAbstractionModuleArgsRelayTransaction {
    pub transaction: AccountAbstractionMetaTransactionData,
    pub options: AccountAbstractionMetaTransactionOptions,
    pub config: Option<AccountAbstractionDeploymentParameters>,
}

// URI: "wrapscan.io/polywrap/account-abstraction-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountAbstractionModuleArgsGetSafeAddress {
    pub config: Option<AccountAbstractionDeploymentParameters>,
}

#[derive(Clone)]
pub struct AccountAbstractionModule {
    uri: Uri,
    invoker: Arc<dyn Invoker>,
    env: Option<Vec<u8>>
}

impl AccountAbstractionModule {
    pub fn new(uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> AccountAbstractionModule {
        let mut config = PolywrapClientConfig::new();
        config.add(SystemClientConfig::default().into());
        config.add(Web3ClientConfig::default().into());
        let client = PolywrapClient::new(config.build());

        let _uri = uri.unwrap_or(uri!("wrapscan.io/polywrap/account-abstraction-kit@0.1.0"));
        let _invoker = invoker.unwrap_or(Arc::new(client));
        let _env = env;

        AccountAbstractionModule {
            uri: _uri,
            invoker: _invoker,
            env: _env,
        }
    }

    pub fn relay_transaction(&self, args: &AccountAbstractionModuleArgsRelayTransaction, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<String, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "relayTransaction",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn get_safe_address(&self, args: &AccountAbstractionModuleArgsGetSafeAddress, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<String, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "getSafeAddress",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }
}
// URI: "wrapscan.io/polywrap/ethers@1.1.1" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersModuleArgsGetChainId {
    pub connection: Option<EthersConnection>,
}

// URI: "wrapscan.io/polywrap/ethers@1.1.1" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersModuleArgsGetSignerAddress {
    pub connection: Option<EthersConnection>,
}

// URI: "wrapscan.io/polywrap/ethers@1.1.1" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersModuleArgsGetSignerBalance {
    #[serde(rename = "blockTag")]
    pub block_tag: Option<BigInt>,
    pub connection: Option<EthersConnection>,
}

// URI: "wrapscan.io/polywrap/ethers@1.1.1" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersModuleArgsGetGasPrice {
    pub connection: Option<EthersConnection>,
}

// URI: "wrapscan.io/polywrap/ethers@1.1.1" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersModuleArgsEstimateEip1559Fees {
    pub connection: Option<EthersConnection>,
}

// URI: "wrapscan.io/polywrap/ethers@1.1.1" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersModuleArgsGetSignerTransactionCount {
    #[serde(rename = "blockTag")]
    pub block_tag: Option<BigInt>,
    pub connection: Option<EthersConnection>,
}

// URI: "wrapscan.io/polywrap/ethers@1.1.1" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersModuleArgsCheckAddress {
    pub address: String,
    pub connection: Option<EthersConnection>,
}

// URI: "wrapscan.io/polywrap/ethers@1.1.1" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersModuleArgsCallContractView {
    pub address: String,
    pub method: String,
    pub args: Option<Vec<String>>,
    pub connection: Option<EthersConnection>,
}

// URI: "wrapscan.io/polywrap/ethers@1.1.1" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersModuleArgsCallContractStatic {
    pub address: String,
    pub method: String,
    pub args: Option<Vec<String>>,
    pub options: Option<EthersTxOptions>,
    pub connection: Option<EthersConnection>,
}

// URI: "wrapscan.io/polywrap/ethers@1.1.1" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersModuleArgsGetBalance {
    pub address: String,
    #[serde(rename = "blockTag")]
    pub block_tag: Option<BigInt>,
    pub connection: Option<EthersConnection>,
}

// URI: "wrapscan.io/polywrap/ethers@1.1.1" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersModuleArgsGetTransaction {
    pub hash: String,
    pub connection: Option<EthersConnection>,
}

// URI: "wrapscan.io/polywrap/ethers@1.1.1" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersModuleArgsSendRpc {
    pub method: String,
    pub params: Vec<String>,
    pub connection: Option<EthersConnection>,
}

// URI: "wrapscan.io/polywrap/ethers@1.1.1" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersModuleArgsEstimateTransactionGas {
    pub tx: EthersTxRequest,
    pub connection: Option<EthersConnection>,
}

// URI: "wrapscan.io/polywrap/ethers@1.1.1" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersModuleArgsAwaitTransaction {
    #[serde(rename = "txHash")]
    pub tx_hash: String,
    pub confirmations: u32,
    pub timeout: Option<u32>,
    pub connection: Option<EthersConnection>,
}

// URI: "wrapscan.io/polywrap/ethers@1.1.1" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersModuleArgsSendTransaction {
    pub tx: EthersTxRequest,
    pub connection: Option<EthersConnection>,
}

// URI: "wrapscan.io/polywrap/ethers@1.1.1" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersModuleArgsSendTransactionAndWait {
    pub tx: EthersTxRequest,
    pub connection: Option<EthersConnection>,
}

// URI: "wrapscan.io/polywrap/ethers@1.1.1" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersModuleArgsDeployContract {
    pub abi: String,
    pub bytecode: String,
    pub args: Option<Vec<String>>,
    pub options: Option<EthersTxOptions>,
    pub connection: Option<EthersConnection>,
}

// URI: "wrapscan.io/polywrap/ethers@1.1.1" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersModuleArgsEstimateContractCallGas {
    pub address: String,
    pub method: String,
    pub args: Option<Vec<String>>,
    pub options: Option<EthersTxOptions>,
    pub connection: Option<EthersConnection>,
}

// URI: "wrapscan.io/polywrap/ethers@1.1.1" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersModuleArgsCallContractMethod {
    pub address: String,
    pub method: String,
    pub args: Option<Vec<String>>,
    pub options: Option<EthersTxOptions>,
    pub connection: Option<EthersConnection>,
}

// URI: "wrapscan.io/polywrap/ethers@1.1.1" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersModuleArgsCallContractMethodAndWait {
    pub address: String,
    pub method: String,
    pub args: Option<Vec<String>>,
    pub options: Option<EthersTxOptions>,
    pub connection: Option<EthersConnection>,
}

// URI: "wrapscan.io/polywrap/ethers@1.1.1" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersModuleArgsSignMessage {
    pub message: String,
    pub connection: Option<EthersConnection>,
}

// URI: "wrapscan.io/polywrap/ethers@1.1.1" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersModuleArgsSignMessageBytes {
    pub bytes: ByteBuf,
    pub connection: Option<EthersConnection>,
}

// URI: "wrapscan.io/polywrap/ethers@1.1.1" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersModuleArgsSignTransaction {
    pub tx: EthersTxRequest,
    pub connection: Option<EthersConnection>,
}

// URI: "wrapscan.io/polywrap/ethers@1.1.1" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersModuleArgsSignTypedData {
    pub payload: JSONString,
    pub connection: Option<EthersConnection>,
}

// URI: "wrapscan.io/polywrap/ethers@1.1.1" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersModuleArgsGenerateCreate2Address {
    pub address: String,
    pub salt: String,
    #[serde(rename = "initCode")]
    pub init_code: String,
}

// URI: "wrapscan.io/polywrap/ethers@1.1.1" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersModuleArgsKeccak256BytesEncodePacked {
    pub value: String,
}

// URI: "wrapscan.io/polywrap/ethers@1.1.1" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersModuleArgsKeccak256 {
    pub value: String,
}

// URI: "wrapscan.io/polywrap/ethers@1.1.1" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersModuleArgsEncodeMetaTransaction {
    pub operation: Option<BigInt>,
    pub to: String,
    pub value: BigInt,
    pub data: String,
}

// URI: "wrapscan.io/polywrap/ethers@1.1.1" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersModuleArgsEncodeParams {
    pub types: Vec<String>,
    pub values: Vec<String>,
}

// URI: "wrapscan.io/polywrap/ethers@1.1.1" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersModuleArgsEncodeFunction {
    pub method: String,
    pub args: Option<Vec<String>>,
}

// URI: "wrapscan.io/polywrap/ethers@1.1.1" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersModuleArgsToWei {
    pub eth: String,
}

// URI: "wrapscan.io/polywrap/ethers@1.1.1" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersModuleArgsToEth {
    pub wei: String,
}

// URI: "wrapscan.io/polywrap/ethers@1.1.1" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersModuleArgsSolidityPack {
    pub types: Vec<String>,
    pub values: Vec<String>,
}

#[derive(Clone)]
pub struct EthersModule {
    uri: Uri,
    invoker: Arc<dyn Invoker>,
    env: Option<Vec<u8>>
}

impl EthersModule {
    pub fn new(uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> EthersModule {
        let mut config = PolywrapClientConfig::new();
        config.add(SystemClientConfig::default().into());
        config.add(Web3ClientConfig::default().into());
        let client = PolywrapClient::new(config.build());

        let _uri = uri.unwrap_or(uri!("wrapscan.io/polywrap/ethers@1.1.1"));
        let _invoker = invoker.unwrap_or(Arc::new(client));
        let _env = env;

        EthersModule {
            uri: _uri,
            invoker: _invoker,
            env: _env,
        }
    }

    pub fn get_chain_id(&self, args: &EthersModuleArgsGetChainId, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<String, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "getChainId",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn get_signer_address(&self, args: &EthersModuleArgsGetSignerAddress, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<String, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "getSignerAddress",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn get_signer_balance(&self, args: &EthersModuleArgsGetSignerBalance, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<BigInt, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "getSignerBalance",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn get_gas_price(&self, args: &EthersModuleArgsGetGasPrice, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<BigInt, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "getGasPrice",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn estimate_eip1559_fees(&self, args: &EthersModuleArgsEstimateEip1559Fees, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<EthersEip1559FeesEstimate, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "estimateEip1559Fees",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn get_signer_transaction_count(&self, args: &EthersModuleArgsGetSignerTransactionCount, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<BigInt, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "getSignerTransactionCount",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn check_address(&self, args: &EthersModuleArgsCheckAddress, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<bool, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "checkAddress",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn call_contract_view(&self, args: &EthersModuleArgsCallContractView, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<String, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "callContractView",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn call_contract_static(&self, args: &EthersModuleArgsCallContractStatic, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<EthersStaticTxResult, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "callContractStatic",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn get_balance(&self, args: &EthersModuleArgsGetBalance, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<BigInt, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "getBalance",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn get_transaction(&self, args: &EthersModuleArgsGetTransaction, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<EthersTxResponse, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "getTransaction",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn send_rpc(&self, args: &EthersModuleArgsSendRpc, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<String, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "sendRpc",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn estimate_transaction_gas(&self, args: &EthersModuleArgsEstimateTransactionGas, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<BigInt, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "estimateTransactionGas",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn await_transaction(&self, args: &EthersModuleArgsAwaitTransaction, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<EthersTxReceipt, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "awaitTransaction",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn send_transaction(&self, args: &EthersModuleArgsSendTransaction, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<EthersTxResponse, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "sendTransaction",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn send_transaction_and_wait(&self, args: &EthersModuleArgsSendTransactionAndWait, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<EthersTxReceipt, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "sendTransactionAndWait",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn deploy_contract(&self, args: &EthersModuleArgsDeployContract, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<String, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "deployContract",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn estimate_contract_call_gas(&self, args: &EthersModuleArgsEstimateContractCallGas, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<BigInt, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "estimateContractCallGas",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn call_contract_method(&self, args: &EthersModuleArgsCallContractMethod, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<EthersTxResponse, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "callContractMethod",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn call_contract_method_and_wait(&self, args: &EthersModuleArgsCallContractMethodAndWait, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<EthersTxReceipt, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "callContractMethodAndWait",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn sign_message(&self, args: &EthersModuleArgsSignMessage, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<String, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "signMessage",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn sign_message_bytes(&self, args: &EthersModuleArgsSignMessageBytes, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<String, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "signMessageBytes",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn sign_transaction(&self, args: &EthersModuleArgsSignTransaction, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<String, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "signTransaction",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn sign_typed_data(&self, args: &EthersModuleArgsSignTypedData, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<String, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "signTypedData",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn generate_create2_address(&self, args: &EthersModuleArgsGenerateCreate2Address, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<String, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "generateCreate2Address",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn keccak256_bytes_encode_packed(&self, args: &EthersModuleArgsKeccak256BytesEncodePacked, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<String, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "keccak256BytesEncodePacked",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn keccak256(&self, args: &EthersModuleArgsKeccak256, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<String, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "keccak256",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn encode_meta_transaction(&self, args: &EthersModuleArgsEncodeMetaTransaction, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<String, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "encodeMetaTransaction",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn encode_params(&self, args: &EthersModuleArgsEncodeParams, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<String, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "encodeParams",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn encode_function(&self, args: &EthersModuleArgsEncodeFunction, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<String, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "encodeFunction",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn to_wei(&self, args: &EthersModuleArgsToWei, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<String, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "toWei",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn to_eth(&self, args: &EthersModuleArgsToEth, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<String, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "toEth",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn solidity_pack(&self, args: &EthersModuleArgsSolidityPack, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<String, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "solidityPack",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }
}
// URI: "wrapscan.io/polywrap/relay-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RelayerModuleArgsGetFeeCollector {
}

// URI: "wrapscan.io/polywrap/relay-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RelayerModuleArgsGetEstimateFee {
    #[serde(rename = "chainId")]
    pub chain_id: i32,
    #[serde(rename = "gasLimit")]
    pub gas_limit: BigInt,
    #[serde(rename = "gasToken")]
    pub gas_token: Option<String>,
}

// URI: "wrapscan.io/polywrap/relay-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RelayerModuleArgsRelayTransaction {
    pub transaction: RelayerRelayTransaction,
}

#[derive(Clone)]
pub struct RelayerModule {
    uri: Uri,
    invoker: Arc<dyn Invoker>,
    env: Option<Vec<u8>>
}

impl RelayerModule {
    pub fn new(uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> RelayerModule {
        let mut config = PolywrapClientConfig::new();
        config.add(SystemClientConfig::default().into());
        config.add(Web3ClientConfig::default().into());
        let client = PolywrapClient::new(config.build());

        let _uri = uri.unwrap_or(uri!("wrapscan.io/polywrap/relay-kit@0.1.0"));
        let _invoker = invoker.unwrap_or(Arc::new(client));
        let _env = env;

        RelayerModule {
            uri: _uri,
            invoker: _invoker,
            env: _env,
        }
    }

    pub fn get_fee_collector(&self, args: &RelayerModuleArgsGetFeeCollector, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<String, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "getFeeCollector",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn get_estimate_fee(&self, args: &RelayerModuleArgsGetEstimateFee, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<BigInt, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "getEstimateFee",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn relay_transaction(&self, args: &RelayerModuleArgsRelayTransaction, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<RelayerRelayResponse, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "relayTransaction",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }
}
// URI: "wrapscan.io/polywrap/protocol-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeModuleArgsCreateProxy {
    pub address: String,
    #[serde(rename = "safeMasterCopyAddress")]
    pub safe_master_copy_address: String,
    pub initializer: String,
    #[serde(rename = "saltNonce")]
    pub salt_nonce: u32,
    pub connection: Option<SafeEthersConnection>,
    #[serde(rename = "txOptions")]
    pub tx_options: Option<SafeEthersTxOptions>,
}

// URI: "wrapscan.io/polywrap/protocol-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeModuleArgsProxyCreationCode {
    pub address: String,
    pub connection: Option<SafeEthersConnection>,
}

// URI: "wrapscan.io/polywrap/protocol-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeModuleArgsGetNonce {
    pub address: String,
    pub connection: Option<SafeEthersConnection>,
}

// URI: "wrapscan.io/polywrap/protocol-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeModuleArgsGetVersion {
    pub address: String,
    pub connection: Option<SafeEthersConnection>,
}

// URI: "wrapscan.io/polywrap/protocol-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeModuleArgsApprovedHashes {
    pub address: String,
    #[serde(rename = "ownerAddress")]
    pub owner_address: String,
    pub hash: String,
    pub connection: Option<SafeEthersConnection>,
}

// URI: "wrapscan.io/polywrap/protocol-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeModuleArgsApproveHash {
    #[serde(rename = "safeAddress")]
    pub safe_address: String,
    pub hash: String,
    pub options: Option<SafeTransactionOptions>,
    pub connection: Option<SafeEthersConnection>,
}

// URI: "wrapscan.io/polywrap/protocol-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeModuleArgsGetSafeContractNetworks {
    pub version: String,
    #[serde(rename = "chainId")]
    pub chain_id: String,
    #[serde(rename = "isL1Safe")]
    pub is_l1_safe: Option<bool>,
    pub filter: Option<SafeContractNetworksFilter>,
}

// URI: "wrapscan.io/polywrap/protocol-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeModuleArgsEncodeExecTransaction {
    #[serde(rename = "safeAddress")]
    pub safe_address: String,
    #[serde(rename = "safeTransaction")]
    pub safe_transaction: SafeSafeTransaction,
}

// URI: "wrapscan.io/polywrap/protocol-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeModuleArgsDeploySafe {
    pub input: SafeDeploymentInput,
    #[serde(rename = "txOptions")]
    pub tx_options: Option<SafeEthersTxOptions>,
}

// URI: "wrapscan.io/polywrap/protocol-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeModuleArgsPredictSafeAddress {
    pub input: SafeDeploymentInput,
}

// URI: "wrapscan.io/polywrap/protocol-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeModuleArgsEncodeDeploySafe {
    pub input: SafeDeploymentInput,
}

// URI: "wrapscan.io/polywrap/protocol-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeModuleArgsSafeIsDeployed {
    #[serde(rename = "safeAddress")]
    pub safe_address: String,
    pub connection: SafeEthersConnection,
}

// URI: "wrapscan.io/polywrap/protocol-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeModuleArgsGetSafeInitializer {
    pub config: SafeSafeAccountConfig,
}

// URI: "wrapscan.io/polywrap/protocol-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeModuleArgsCreateTransaction {
    pub tx: SafeSafeTransactionDataPartial,
}

// URI: "wrapscan.io/polywrap/protocol-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeModuleArgsCreateMultiSendTransaction {
    pub txs: Vec<SafeSafeTransactionDataPartial>,
    pub options: Option<SafeSafeTransactionOptions>,
    #[serde(rename = "onlyCalls")]
    pub only_calls: Option<bool>,
    #[serde(rename = "customMultiSendContractAddress")]
    pub custom_multi_send_contract_address: Option<String>,
}

// URI: "wrapscan.io/polywrap/protocol-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeModuleArgsAddSignature {
    pub tx: SafeSafeTransaction,
    #[serde(rename = "signingMethod")]
    pub signing_method: Option<String>,
}

// URI: "wrapscan.io/polywrap/protocol-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeModuleArgsGetTransactionHash {
    pub tx: SafeSafeTransactionData,
}

// URI: "wrapscan.io/polywrap/protocol-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeModuleArgsSignTransactionHash {
    pub hash: String,
}

// URI: "wrapscan.io/polywrap/protocol-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeModuleArgsApproveTransactionHash {
    pub hash: String,
    pub options: Option<SafeTransactionOptions>,
}

// URI: "wrapscan.io/polywrap/protocol-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeModuleArgsExecuteTransaction {
    pub tx: SafeSafeTransaction,
    pub options: Option<SafeTransactionOptions>,
}

// URI: "wrapscan.io/polywrap/protocol-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeModuleArgsGetOwnersWhoApprovedTx {
    pub hash: String,
}

// URI: "wrapscan.io/polywrap/protocol-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeModuleArgsGetSignature {
    pub tx: SafeSafeTransaction,
    #[serde(rename = "signingMethod")]
    pub signing_method: Option<String>,
    #[serde(rename = "safeAddress")]
    pub safe_address: String,
    pub connection: SafeEthersConnection,
}

// URI: "wrapscan.io/polywrap/protocol-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeModuleArgsGetOwners {
    #[serde(rename = "safeAddress")]
    pub safe_address: String,
    pub connection: Option<SafeEthersConnection>,
}

// URI: "wrapscan.io/polywrap/protocol-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeModuleArgsGetThreshold {
    #[serde(rename = "safeAddress")]
    pub safe_address: String,
    pub connection: Option<SafeEthersConnection>,
}

// URI: "wrapscan.io/polywrap/protocol-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeModuleArgsIsOwner {
    #[serde(rename = "ownerAddress")]
    pub owner_address: String,
    #[serde(rename = "safeAddress")]
    pub safe_address: String,
    pub connection: Option<SafeEthersConnection>,
}

// URI: "wrapscan.io/polywrap/protocol-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeModuleArgsGetModules {
}

// URI: "wrapscan.io/polywrap/protocol-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeModuleArgsIsModuleEnabled {
    #[serde(rename = "moduleAddress")]
    pub module_address: String,
}

// URI: "wrapscan.io/polywrap/protocol-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeModuleArgsEncodeEnableModuleData {
    #[serde(rename = "moduleAddress")]
    pub module_address: String,
}

// URI: "wrapscan.io/polywrap/protocol-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeModuleArgsEncodeDisableModuleData {
    #[serde(rename = "moduleAddress")]
    pub module_address: String,
}

// URI: "wrapscan.io/polywrap/protocol-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeModuleArgsEncodeMultiSendData {
    pub txs: Vec<SafeSafeTransactionDataPartial>,
}

// URI: "wrapscan.io/polywrap/protocol-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeModuleArgsEncodeAddOwnerWithThresholdData {
    #[serde(rename = "ownerAddress")]
    pub owner_address: String,
    pub threshold: Option<u32>,
}

// URI: "wrapscan.io/polywrap/protocol-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeModuleArgsEncodeRemoveOwnerData {
    #[serde(rename = "ownerAddress")]
    pub owner_address: String,
    pub threshold: Option<u32>,
}

// URI: "wrapscan.io/polywrap/protocol-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeModuleArgsEncodeSwapOwnerData {
    #[serde(rename = "oldOwnerAddress")]
    pub old_owner_address: String,
    #[serde(rename = "newOwnerAddress")]
    pub new_owner_address: String,
}

// URI: "wrapscan.io/polywrap/protocol-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeModuleArgsEncodeChangeThresholdData {
    pub threshold: u32,
}

#[derive(Clone)]
pub struct SafeModule {
    uri: Uri,
    invoker: Arc<dyn Invoker>,
    env: Option<Vec<u8>>
}

impl SafeModule {
    pub fn new(uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> SafeModule {
        let mut config = PolywrapClientConfig::new();
        config.add(SystemClientConfig::default().into());
        config.add(Web3ClientConfig::default().into());
        let client = PolywrapClient::new(config.build());

        let _uri = uri.unwrap_or(uri!("wrapscan.io/polywrap/protocol-kit@0.1.0"));
        let _invoker = invoker.unwrap_or(Arc::new(client));
        let _env = env;

        SafeModule {
            uri: _uri,
            invoker: _invoker,
            env: _env,
        }
    }

    pub fn create_proxy(&self, args: &SafeModuleArgsCreateProxy, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<String, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "createProxy",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn proxy_creation_code(&self, args: &SafeModuleArgsProxyCreationCode, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<String, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "proxyCreationCode",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn get_nonce(&self, args: &SafeModuleArgsGetNonce, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<BigInt, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "getNonce",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn get_version(&self, args: &SafeModuleArgsGetVersion, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<String, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "getVersion",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn approved_hashes(&self, args: &SafeModuleArgsApprovedHashes, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<BigInt, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "approvedHashes",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn approve_hash(&self, args: &SafeModuleArgsApproveHash, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<SafeEthersTxReceipt, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "approveHash",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn get_safe_contract_networks(&self, args: &SafeModuleArgsGetSafeContractNetworks, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<SafeContractNetworksConfig, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "getSafeContractNetworks",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn encode_exec_transaction(&self, args: &SafeModuleArgsEncodeExecTransaction, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<String, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "encodeExecTransaction",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn deploy_safe(&self, args: &SafeModuleArgsDeploySafe, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<String, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "deploySafe",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn predict_safe_address(&self, args: &SafeModuleArgsPredictSafeAddress, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<String, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "predictSafeAddress",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn encode_deploy_safe(&self, args: &SafeModuleArgsEncodeDeploySafe, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<String, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "encodeDeploySafe",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn safe_is_deployed(&self, args: &SafeModuleArgsSafeIsDeployed, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<bool, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "safeIsDeployed",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn get_safe_initializer(&self, args: &SafeModuleArgsGetSafeInitializer, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<String, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "getSafeInitializer",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn create_transaction(&self, args: &SafeModuleArgsCreateTransaction, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<SafeSafeTransaction, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "createTransaction",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn create_multi_send_transaction(&self, args: &SafeModuleArgsCreateMultiSendTransaction, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<SafeSafeTransaction, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "createMultiSendTransaction",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn add_signature(&self, args: &SafeModuleArgsAddSignature, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<SafeSafeTransaction, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "addSignature",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn get_transaction_hash(&self, args: &SafeModuleArgsGetTransactionHash, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<String, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "getTransactionHash",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn sign_transaction_hash(&self, args: &SafeModuleArgsSignTransactionHash, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<SafeSafeSignature, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "signTransactionHash",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn approve_transaction_hash(&self, args: &SafeModuleArgsApproveTransactionHash, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<SafeEthersTxReceipt, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "approveTransactionHash",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn execute_transaction(&self, args: &SafeModuleArgsExecuteTransaction, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<SafeEthersTxReceipt, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "executeTransaction",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn get_owners_who_approved_tx(&self, args: &SafeModuleArgsGetOwnersWhoApprovedTx, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<Vec<String>, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "getOwnersWhoApprovedTx",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn get_signature(&self, args: &SafeModuleArgsGetSignature, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<SafeSafeTransaction, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "getSignature",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn get_owners(&self, args: &SafeModuleArgsGetOwners, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<Vec<String>, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "getOwners",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn get_threshold(&self, args: &SafeModuleArgsGetThreshold, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<u32, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "getThreshold",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn is_owner(&self, args: &SafeModuleArgsIsOwner, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<bool, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "isOwner",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn get_modules(&self, args: &SafeModuleArgsGetModules, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<Vec<String>, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "getModules",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn is_module_enabled(&self, args: &SafeModuleArgsIsModuleEnabled, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<bool, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "isModuleEnabled",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn encode_enable_module_data(&self, args: &SafeModuleArgsEncodeEnableModuleData, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<String, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "encodeEnableModuleData",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn encode_disable_module_data(&self, args: &SafeModuleArgsEncodeDisableModuleData, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<String, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "encodeDisableModuleData",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn encode_multi_send_data(&self, args: &SafeModuleArgsEncodeMultiSendData, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<String, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "encodeMultiSendData",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn encode_add_owner_with_threshold_data(&self, args: &SafeModuleArgsEncodeAddOwnerWithThresholdData, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<String, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "encodeAddOwnerWithThresholdData",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn encode_remove_owner_data(&self, args: &SafeModuleArgsEncodeRemoveOwnerData, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<String, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "encodeRemoveOwnerData",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn encode_swap_owner_data(&self, args: &SafeModuleArgsEncodeSwapOwnerData, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<String, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "encodeSwapOwnerData",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn encode_change_threshold_data(&self, args: &SafeModuleArgsEncodeChangeThresholdData, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<String, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match &env {
            Some(e) => Some(e.as_slice()),
            None => match &self.env {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "encodeChangeThresholdData",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }
}
// Imported Modules END //
