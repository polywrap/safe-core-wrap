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

#[derive(Clone)]
pub struct InvokeOptions {
    pub uri: Option<Uri>,
    pub client: Option<Arc<dyn Invoker>>,
    pub env: Option<Vec<u8>> 
}

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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountAbstractionEnv {
    pub connection: Option<AccountAbstractionEthersConnection>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RelayerEnv {
    #[serde(rename = "relayerApiKey")]
    pub relayer_api_key: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeEnv {
    #[serde(rename = "safeAddress")]
    pub safe_address: String,
    pub connection: SafeEthersConnection,
}
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
pub struct AccountAbstractionArgsRelayTransaction {
    pub transaction: AccountAbstractionMetaTransactionData,
    pub options: AccountAbstractionMetaTransactionOptions,
    pub config: Option<AccountAbstractionDeploymentParameters>,
}

// URI: "wrapscan.io/polywrap/account-abstraction-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountAbstractionArgsGetSafeAddress {
    pub config: Option<AccountAbstractionDeploymentParameters>,
}

#[derive(Clone)]
pub struct AccountAbstraction {
    pub uri: Uri,
    pub invoker: Arc<dyn Invoker>,
    pub env: Option<Vec<u8>>
}

impl AccountAbstraction {
    pub fn new(invoke_options: Option<InvokeOptions>) -> AccountAbstraction {
        let mut config = PolywrapClientConfig::new();
        config.add(SystemClientConfig::default().into());
        config.add(Web3ClientConfig::default().into());
        let client = PolywrapClient::new(config.build());

        let default_client = Arc::new(client);
        let default_uri = uri!("wrapscan.io/polywrap/account-abstraction-kit@0.1.0");
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let _uri = if let Some(uri) = invoke_option.uri {
                uri
            } else {
                default_uri
            };

            let _invoker = if let Some(invoker) = invoke_option.client {
                invoker
            } else {
                default_client
            };

            (_uri, _invoker, invoke_option.env)
        } else {
            (default_uri, default_client as Arc<dyn Invoker>, None)
        };

        AccountAbstraction {
            uri: _uri,
            invoker: _invoker,
            env: _env,
        }
    }

    pub fn default_uri() -> Uri {
        uri!("wrapscan.io/polywrap/account-abstraction-kit@0.1.0")
    }

    pub fn relay_transaction(&self, args: &AccountAbstractionArgsRelayTransaction, invoke_options: Option<InvokeOptions>) -> Result<String, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "relayTransaction",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn get_safe_address(&self, args: &AccountAbstractionArgsGetSafeAddress, invoke_options: Option<InvokeOptions>) -> Result<String, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "getSafeAddress",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }
}
// URI: "wrapscan.io/polywrap/ethers@1.1.1" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersArgsGetChainId {
    pub connection: Option<EthersConnection>,
}

// URI: "wrapscan.io/polywrap/ethers@1.1.1" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersArgsGetSignerAddress {
    pub connection: Option<EthersConnection>,
}

// URI: "wrapscan.io/polywrap/ethers@1.1.1" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersArgsGetSignerBalance {
    #[serde(rename = "blockTag")]
    pub block_tag: Option<BigInt>,
    pub connection: Option<EthersConnection>,
}

// URI: "wrapscan.io/polywrap/ethers@1.1.1" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersArgsGetGasPrice {
    pub connection: Option<EthersConnection>,
}

// URI: "wrapscan.io/polywrap/ethers@1.1.1" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersArgsEstimateEip1559Fees {
    pub connection: Option<EthersConnection>,
}

// URI: "wrapscan.io/polywrap/ethers@1.1.1" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersArgsGetSignerTransactionCount {
    #[serde(rename = "blockTag")]
    pub block_tag: Option<BigInt>,
    pub connection: Option<EthersConnection>,
}

// URI: "wrapscan.io/polywrap/ethers@1.1.1" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersArgsCheckAddress {
    pub address: String,
    pub connection: Option<EthersConnection>,
}

// URI: "wrapscan.io/polywrap/ethers@1.1.1" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersArgsCallContractView {
    pub address: String,
    pub method: String,
    pub args: Option<Vec<String>>,
    pub connection: Option<EthersConnection>,
}

// URI: "wrapscan.io/polywrap/ethers@1.1.1" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersArgsCallContractStatic {
    pub address: String,
    pub method: String,
    pub args: Option<Vec<String>>,
    pub options: Option<EthersTxOptions>,
    pub connection: Option<EthersConnection>,
}

// URI: "wrapscan.io/polywrap/ethers@1.1.1" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersArgsGetBalance {
    pub address: String,
    #[serde(rename = "blockTag")]
    pub block_tag: Option<BigInt>,
    pub connection: Option<EthersConnection>,
}

// URI: "wrapscan.io/polywrap/ethers@1.1.1" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersArgsGetTransaction {
    pub hash: String,
    pub connection: Option<EthersConnection>,
}

// URI: "wrapscan.io/polywrap/ethers@1.1.1" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersArgsSendRpc {
    pub method: String,
    pub params: Vec<String>,
    pub connection: Option<EthersConnection>,
}

// URI: "wrapscan.io/polywrap/ethers@1.1.1" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersArgsEstimateTransactionGas {
    pub tx: EthersTxRequest,
    pub connection: Option<EthersConnection>,
}

// URI: "wrapscan.io/polywrap/ethers@1.1.1" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersArgsAwaitTransaction {
    #[serde(rename = "txHash")]
    pub tx_hash: String,
    pub confirmations: u32,
    pub timeout: Option<u32>,
    pub connection: Option<EthersConnection>,
}

// URI: "wrapscan.io/polywrap/ethers@1.1.1" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersArgsSendTransaction {
    pub tx: EthersTxRequest,
    pub connection: Option<EthersConnection>,
}

// URI: "wrapscan.io/polywrap/ethers@1.1.1" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersArgsSendTransactionAndWait {
    pub tx: EthersTxRequest,
    pub connection: Option<EthersConnection>,
}

// URI: "wrapscan.io/polywrap/ethers@1.1.1" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersArgsDeployContract {
    pub abi: String,
    pub bytecode: String,
    pub args: Option<Vec<String>>,
    pub options: Option<EthersTxOptions>,
    pub connection: Option<EthersConnection>,
}

// URI: "wrapscan.io/polywrap/ethers@1.1.1" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersArgsEstimateContractCallGas {
    pub address: String,
    pub method: String,
    pub args: Option<Vec<String>>,
    pub options: Option<EthersTxOptions>,
    pub connection: Option<EthersConnection>,
}

// URI: "wrapscan.io/polywrap/ethers@1.1.1" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersArgsCallContractMethod {
    pub address: String,
    pub method: String,
    pub args: Option<Vec<String>>,
    pub options: Option<EthersTxOptions>,
    pub connection: Option<EthersConnection>,
}

// URI: "wrapscan.io/polywrap/ethers@1.1.1" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersArgsCallContractMethodAndWait {
    pub address: String,
    pub method: String,
    pub args: Option<Vec<String>>,
    pub options: Option<EthersTxOptions>,
    pub connection: Option<EthersConnection>,
}

// URI: "wrapscan.io/polywrap/ethers@1.1.1" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersArgsSignMessage {
    pub message: String,
    pub connection: Option<EthersConnection>,
}

// URI: "wrapscan.io/polywrap/ethers@1.1.1" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersArgsSignMessageBytes {
    pub bytes: ByteBuf,
    pub connection: Option<EthersConnection>,
}

// URI: "wrapscan.io/polywrap/ethers@1.1.1" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersArgsSignTransaction {
    pub tx: EthersTxRequest,
    pub connection: Option<EthersConnection>,
}

// URI: "wrapscan.io/polywrap/ethers@1.1.1" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersArgsSignTypedData {
    pub payload: JSONString,
    pub connection: Option<EthersConnection>,
}

// URI: "wrapscan.io/polywrap/ethers@1.1.1" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersArgsGenerateCreate2Address {
    pub address: String,
    pub salt: String,
    #[serde(rename = "initCode")]
    pub init_code: String,
}

// URI: "wrapscan.io/polywrap/ethers@1.1.1" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersArgsKeccak256BytesEncodePacked {
    pub value: String,
}

// URI: "wrapscan.io/polywrap/ethers@1.1.1" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersArgsKeccak256 {
    pub value: String,
}

// URI: "wrapscan.io/polywrap/ethers@1.1.1" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersArgsEncodeMetaTransaction {
    pub operation: Option<BigInt>,
    pub to: String,
    pub value: BigInt,
    pub data: String,
}

// URI: "wrapscan.io/polywrap/ethers@1.1.1" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersArgsEncodeParams {
    pub types: Vec<String>,
    pub values: Vec<String>,
}

// URI: "wrapscan.io/polywrap/ethers@1.1.1" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersArgsEncodeFunction {
    pub method: String,
    pub args: Option<Vec<String>>,
}

// URI: "wrapscan.io/polywrap/ethers@1.1.1" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersArgsToWei {
    pub eth: String,
}

// URI: "wrapscan.io/polywrap/ethers@1.1.1" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersArgsToEth {
    pub wei: String,
}

// URI: "wrapscan.io/polywrap/ethers@1.1.1" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EthersArgsSolidityPack {
    pub types: Vec<String>,
    pub values: Vec<String>,
}

#[derive(Clone)]
pub struct Ethers {
    pub uri: Uri,
    pub invoker: Arc<dyn Invoker>,
    pub env: Option<Vec<u8>>
}

impl Ethers {
    pub fn new(invoke_options: Option<InvokeOptions>) -> Ethers {
        let mut config = PolywrapClientConfig::new();
        config.add(SystemClientConfig::default().into());
        config.add(Web3ClientConfig::default().into());
        let client = PolywrapClient::new(config.build());

        let default_client = Arc::new(client);
        let default_uri = uri!("wrapscan.io/polywrap/ethers@1.1.1");
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let _uri = if let Some(uri) = invoke_option.uri {
                uri
            } else {
                default_uri
            };

            let _invoker = if let Some(invoker) = invoke_option.client {
                invoker
            } else {
                default_client
            };

            (_uri, _invoker, invoke_option.env)
        } else {
            (default_uri, default_client as Arc<dyn Invoker>, None)
        };

        Ethers {
            uri: _uri,
            invoker: _invoker,
            env: _env,
        }
    }

    pub fn default_uri() -> Uri {
        uri!("wrapscan.io/polywrap/ethers@1.1.1")
    }

    pub fn get_chain_id(&self, args: &EthersArgsGetChainId, invoke_options: Option<InvokeOptions>) -> Result<String, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "getChainId",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn get_signer_address(&self, args: &EthersArgsGetSignerAddress, invoke_options: Option<InvokeOptions>) -> Result<String, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "getSignerAddress",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn get_signer_balance(&self, args: &EthersArgsGetSignerBalance, invoke_options: Option<InvokeOptions>) -> Result<BigInt, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "getSignerBalance",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn get_gas_price(&self, args: &EthersArgsGetGasPrice, invoke_options: Option<InvokeOptions>) -> Result<BigInt, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "getGasPrice",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn estimate_eip1559_fees(&self, args: &EthersArgsEstimateEip1559Fees, invoke_options: Option<InvokeOptions>) -> Result<EthersEip1559FeesEstimate, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "estimateEip1559Fees",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn get_signer_transaction_count(&self, args: &EthersArgsGetSignerTransactionCount, invoke_options: Option<InvokeOptions>) -> Result<BigInt, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "getSignerTransactionCount",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn check_address(&self, args: &EthersArgsCheckAddress, invoke_options: Option<InvokeOptions>) -> Result<bool, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "checkAddress",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn call_contract_view(&self, args: &EthersArgsCallContractView, invoke_options: Option<InvokeOptions>) -> Result<String, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "callContractView",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn call_contract_static(&self, args: &EthersArgsCallContractStatic, invoke_options: Option<InvokeOptions>) -> Result<EthersStaticTxResult, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "callContractStatic",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn get_balance(&self, args: &EthersArgsGetBalance, invoke_options: Option<InvokeOptions>) -> Result<BigInt, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "getBalance",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn get_transaction(&self, args: &EthersArgsGetTransaction, invoke_options: Option<InvokeOptions>) -> Result<EthersTxResponse, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "getTransaction",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn send_rpc(&self, args: &EthersArgsSendRpc, invoke_options: Option<InvokeOptions>) -> Result<String, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "sendRpc",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn estimate_transaction_gas(&self, args: &EthersArgsEstimateTransactionGas, invoke_options: Option<InvokeOptions>) -> Result<BigInt, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "estimateTransactionGas",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn await_transaction(&self, args: &EthersArgsAwaitTransaction, invoke_options: Option<InvokeOptions>) -> Result<EthersTxReceipt, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "awaitTransaction",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn send_transaction(&self, args: &EthersArgsSendTransaction, invoke_options: Option<InvokeOptions>) -> Result<EthersTxResponse, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "sendTransaction",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn send_transaction_and_wait(&self, args: &EthersArgsSendTransactionAndWait, invoke_options: Option<InvokeOptions>) -> Result<EthersTxReceipt, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "sendTransactionAndWait",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn deploy_contract(&self, args: &EthersArgsDeployContract, invoke_options: Option<InvokeOptions>) -> Result<String, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "deployContract",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn estimate_contract_call_gas(&self, args: &EthersArgsEstimateContractCallGas, invoke_options: Option<InvokeOptions>) -> Result<BigInt, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "estimateContractCallGas",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn call_contract_method(&self, args: &EthersArgsCallContractMethod, invoke_options: Option<InvokeOptions>) -> Result<EthersTxResponse, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "callContractMethod",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn call_contract_method_and_wait(&self, args: &EthersArgsCallContractMethodAndWait, invoke_options: Option<InvokeOptions>) -> Result<EthersTxReceipt, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "callContractMethodAndWait",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn sign_message(&self, args: &EthersArgsSignMessage, invoke_options: Option<InvokeOptions>) -> Result<String, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "signMessage",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn sign_message_bytes(&self, args: &EthersArgsSignMessageBytes, invoke_options: Option<InvokeOptions>) -> Result<String, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "signMessageBytes",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn sign_transaction(&self, args: &EthersArgsSignTransaction, invoke_options: Option<InvokeOptions>) -> Result<String, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "signTransaction",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn sign_typed_data(&self, args: &EthersArgsSignTypedData, invoke_options: Option<InvokeOptions>) -> Result<String, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "signTypedData",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn generate_create2_address(&self, args: &EthersArgsGenerateCreate2Address, invoke_options: Option<InvokeOptions>) -> Result<String, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "generateCreate2Address",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn keccak256_bytes_encode_packed(&self, args: &EthersArgsKeccak256BytesEncodePacked, invoke_options: Option<InvokeOptions>) -> Result<String, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "keccak256BytesEncodePacked",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn keccak256(&self, args: &EthersArgsKeccak256, invoke_options: Option<InvokeOptions>) -> Result<String, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "keccak256",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn encode_meta_transaction(&self, args: &EthersArgsEncodeMetaTransaction, invoke_options: Option<InvokeOptions>) -> Result<String, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "encodeMetaTransaction",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn encode_params(&self, args: &EthersArgsEncodeParams, invoke_options: Option<InvokeOptions>) -> Result<String, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "encodeParams",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn encode_function(&self, args: &EthersArgsEncodeFunction, invoke_options: Option<InvokeOptions>) -> Result<String, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "encodeFunction",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn to_wei(&self, args: &EthersArgsToWei, invoke_options: Option<InvokeOptions>) -> Result<String, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "toWei",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn to_eth(&self, args: &EthersArgsToEth, invoke_options: Option<InvokeOptions>) -> Result<String, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "toEth",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn solidity_pack(&self, args: &EthersArgsSolidityPack, invoke_options: Option<InvokeOptions>) -> Result<String, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "solidityPack",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }
}
// URI: "wrapscan.io/polywrap/relay-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RelayerArgsGetFeeCollector {
}

// URI: "wrapscan.io/polywrap/relay-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RelayerArgsGetEstimateFee {
    #[serde(rename = "chainId")]
    pub chain_id: i32,
    #[serde(rename = "gasLimit")]
    pub gas_limit: BigInt,
    #[serde(rename = "gasToken")]
    pub gas_token: Option<String>,
}

// URI: "wrapscan.io/polywrap/relay-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RelayerArgsRelayTransaction {
    pub transaction: RelayerRelayTransaction,
}

#[derive(Clone)]
pub struct Relayer {
    pub uri: Uri,
    pub invoker: Arc<dyn Invoker>,
    pub env: Option<Vec<u8>>
}

impl Relayer {
    pub fn new(invoke_options: Option<InvokeOptions>) -> Relayer {
        let mut config = PolywrapClientConfig::new();
        config.add(SystemClientConfig::default().into());
        config.add(Web3ClientConfig::default().into());
        let client = PolywrapClient::new(config.build());

        let default_client = Arc::new(client);
        let default_uri = uri!("wrapscan.io/polywrap/relay-kit@0.1.0");
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let _uri = if let Some(uri) = invoke_option.uri {
                uri
            } else {
                default_uri
            };

            let _invoker = if let Some(invoker) = invoke_option.client {
                invoker
            } else {
                default_client
            };

            (_uri, _invoker, invoke_option.env)
        } else {
            (default_uri, default_client as Arc<dyn Invoker>, None)
        };

        Relayer {
            uri: _uri,
            invoker: _invoker,
            env: _env,
        }
    }

    pub fn default_uri() -> Uri {
        uri!("wrapscan.io/polywrap/relay-kit@0.1.0")
    }

    pub fn get_fee_collector(&self, args: &RelayerArgsGetFeeCollector, invoke_options: Option<InvokeOptions>) -> Result<String, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "getFeeCollector",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn get_estimate_fee(&self, args: &RelayerArgsGetEstimateFee, invoke_options: Option<InvokeOptions>) -> Result<BigInt, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "getEstimateFee",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn relay_transaction(&self, args: &RelayerArgsRelayTransaction, invoke_options: Option<InvokeOptions>) -> Result<RelayerRelayResponse, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "relayTransaction",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }
}
// URI: "wrapscan.io/polywrap/protocol-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeArgsCreateProxy {
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
pub struct SafeArgsProxyCreationCode {
    pub address: String,
    pub connection: Option<SafeEthersConnection>,
}

// URI: "wrapscan.io/polywrap/protocol-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeArgsGetNonce {
    pub address: String,
    pub connection: Option<SafeEthersConnection>,
}

// URI: "wrapscan.io/polywrap/protocol-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeArgsGetVersion {
    pub address: String,
    pub connection: Option<SafeEthersConnection>,
}

// URI: "wrapscan.io/polywrap/protocol-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeArgsApprovedHashes {
    pub address: String,
    #[serde(rename = "ownerAddress")]
    pub owner_address: String,
    pub hash: String,
    pub connection: Option<SafeEthersConnection>,
}

// URI: "wrapscan.io/polywrap/protocol-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeArgsApproveHash {
    #[serde(rename = "safeAddress")]
    pub safe_address: String,
    pub hash: String,
    pub options: Option<SafeTransactionOptions>,
    pub connection: Option<SafeEthersConnection>,
}

// URI: "wrapscan.io/polywrap/protocol-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeArgsGetSafeContractNetworks {
    pub version: String,
    #[serde(rename = "chainId")]
    pub chain_id: String,
    #[serde(rename = "isL1Safe")]
    pub is_l1_safe: Option<bool>,
    pub filter: Option<SafeContractNetworksFilter>,
}

// URI: "wrapscan.io/polywrap/protocol-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeArgsEncodeExecTransaction {
    #[serde(rename = "safeAddress")]
    pub safe_address: String,
    #[serde(rename = "safeTransaction")]
    pub safe_transaction: SafeSafeTransaction,
}

// URI: "wrapscan.io/polywrap/protocol-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeArgsDeploySafe {
    pub input: SafeDeploymentInput,
    #[serde(rename = "txOptions")]
    pub tx_options: Option<SafeEthersTxOptions>,
}

// URI: "wrapscan.io/polywrap/protocol-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeArgsPredictSafeAddress {
    pub input: SafeDeploymentInput,
}

// URI: "wrapscan.io/polywrap/protocol-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeArgsEncodeDeploySafe {
    pub input: SafeDeploymentInput,
}

// URI: "wrapscan.io/polywrap/protocol-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeArgsSafeIsDeployed {
    #[serde(rename = "safeAddress")]
    pub safe_address: String,
    pub connection: SafeEthersConnection,
}

// URI: "wrapscan.io/polywrap/protocol-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeArgsGetSafeInitializer {
    pub config: SafeSafeAccountConfig,
}

// URI: "wrapscan.io/polywrap/protocol-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeArgsCreateTransaction {
    pub tx: SafeSafeTransactionDataPartial,
}

// URI: "wrapscan.io/polywrap/protocol-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeArgsCreateMultiSendTransaction {
    pub txs: Vec<SafeSafeTransactionDataPartial>,
    pub options: Option<SafeSafeTransactionOptions>,
    #[serde(rename = "onlyCalls")]
    pub only_calls: Option<bool>,
    #[serde(rename = "customMultiSendContractAddress")]
    pub custom_multi_send_contract_address: Option<String>,
}

// URI: "wrapscan.io/polywrap/protocol-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeArgsAddSignature {
    pub tx: SafeSafeTransaction,
    #[serde(rename = "signingMethod")]
    pub signing_method: Option<String>,
}

// URI: "wrapscan.io/polywrap/protocol-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeArgsGetTransactionHash {
    pub tx: SafeSafeTransactionData,
}

// URI: "wrapscan.io/polywrap/protocol-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeArgsSignTransactionHash {
    pub hash: String,
}

// URI: "wrapscan.io/polywrap/protocol-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeArgsApproveTransactionHash {
    pub hash: String,
    pub options: Option<SafeTransactionOptions>,
}

// URI: "wrapscan.io/polywrap/protocol-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeArgsExecuteTransaction {
    pub tx: SafeSafeTransaction,
    pub options: Option<SafeTransactionOptions>,
}

// URI: "wrapscan.io/polywrap/protocol-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeArgsGetOwnersWhoApprovedTx {
    pub hash: String,
}

// URI: "wrapscan.io/polywrap/protocol-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeArgsGetSignature {
    pub tx: SafeSafeTransaction,
    #[serde(rename = "signingMethod")]
    pub signing_method: Option<String>,
    #[serde(rename = "safeAddress")]
    pub safe_address: String,
    pub connection: SafeEthersConnection,
}

// URI: "wrapscan.io/polywrap/protocol-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeArgsGetOwners {
    #[serde(rename = "safeAddress")]
    pub safe_address: String,
    pub connection: Option<SafeEthersConnection>,
}

// URI: "wrapscan.io/polywrap/protocol-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeArgsGetThreshold {
    #[serde(rename = "safeAddress")]
    pub safe_address: String,
    pub connection: Option<SafeEthersConnection>,
}

// URI: "wrapscan.io/polywrap/protocol-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeArgsIsOwner {
    #[serde(rename = "ownerAddress")]
    pub owner_address: String,
    #[serde(rename = "safeAddress")]
    pub safe_address: String,
    pub connection: Option<SafeEthersConnection>,
}

// URI: "wrapscan.io/polywrap/protocol-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeArgsGetModules {
}

// URI: "wrapscan.io/polywrap/protocol-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeArgsIsModuleEnabled {
    #[serde(rename = "moduleAddress")]
    pub module_address: String,
}

// URI: "wrapscan.io/polywrap/protocol-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeArgsEncodeEnableModuleData {
    #[serde(rename = "moduleAddress")]
    pub module_address: String,
}

// URI: "wrapscan.io/polywrap/protocol-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeArgsEncodeDisableModuleData {
    #[serde(rename = "moduleAddress")]
    pub module_address: String,
}

// URI: "wrapscan.io/polywrap/protocol-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeArgsEncodeMultiSendData {
    pub txs: Vec<SafeSafeTransactionDataPartial>,
}

// URI: "wrapscan.io/polywrap/protocol-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeArgsEncodeAddOwnerWithThresholdData {
    #[serde(rename = "ownerAddress")]
    pub owner_address: String,
    pub threshold: Option<u32>,
}

// URI: "wrapscan.io/polywrap/protocol-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeArgsEncodeRemoveOwnerData {
    #[serde(rename = "ownerAddress")]
    pub owner_address: String,
    pub threshold: Option<u32>,
}

// URI: "wrapscan.io/polywrap/protocol-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeArgsEncodeSwapOwnerData {
    #[serde(rename = "oldOwnerAddress")]
    pub old_owner_address: String,
    #[serde(rename = "newOwnerAddress")]
    pub new_owner_address: String,
}

// URI: "wrapscan.io/polywrap/protocol-kit@0.1.0" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafeArgsEncodeChangeThresholdData {
    pub threshold: u32,
}

#[derive(Clone)]
pub struct Safe {
    pub uri: Uri,
    pub invoker: Arc<dyn Invoker>,
    pub env: Option<Vec<u8>>
}

impl Safe {
    pub fn new(invoke_options: Option<InvokeOptions>) -> Safe {
        let mut config = PolywrapClientConfig::new();
        config.add(SystemClientConfig::default().into());
        config.add(Web3ClientConfig::default().into());
        let client = PolywrapClient::new(config.build());

        let default_client = Arc::new(client);
        let default_uri = uri!("wrapscan.io/polywrap/protocol-kit@0.1.0");
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let _uri = if let Some(uri) = invoke_option.uri {
                uri
            } else {
                default_uri
            };

            let _invoker = if let Some(invoker) = invoke_option.client {
                invoker
            } else {
                default_client
            };

            (_uri, _invoker, invoke_option.env)
        } else {
            (default_uri, default_client as Arc<dyn Invoker>, None)
        };

        Safe {
            uri: _uri,
            invoker: _invoker,
            env: _env,
        }
    }

    pub fn default_uri() -> Uri {
        uri!("wrapscan.io/polywrap/protocol-kit@0.1.0")
    }

    pub fn create_proxy(&self, args: &SafeArgsCreateProxy, invoke_options: Option<InvokeOptions>) -> Result<String, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "createProxy",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn proxy_creation_code(&self, args: &SafeArgsProxyCreationCode, invoke_options: Option<InvokeOptions>) -> Result<String, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "proxyCreationCode",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn get_nonce(&self, args: &SafeArgsGetNonce, invoke_options: Option<InvokeOptions>) -> Result<BigInt, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "getNonce",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn get_version(&self, args: &SafeArgsGetVersion, invoke_options: Option<InvokeOptions>) -> Result<String, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "getVersion",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn approved_hashes(&self, args: &SafeArgsApprovedHashes, invoke_options: Option<InvokeOptions>) -> Result<BigInt, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "approvedHashes",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn approve_hash(&self, args: &SafeArgsApproveHash, invoke_options: Option<InvokeOptions>) -> Result<SafeEthersTxReceipt, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "approveHash",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn get_safe_contract_networks(&self, args: &SafeArgsGetSafeContractNetworks, invoke_options: Option<InvokeOptions>) -> Result<SafeContractNetworksConfig, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "getSafeContractNetworks",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn encode_exec_transaction(&self, args: &SafeArgsEncodeExecTransaction, invoke_options: Option<InvokeOptions>) -> Result<String, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "encodeExecTransaction",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn deploy_safe(&self, args: &SafeArgsDeploySafe, invoke_options: Option<InvokeOptions>) -> Result<String, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "deploySafe",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn predict_safe_address(&self, args: &SafeArgsPredictSafeAddress, invoke_options: Option<InvokeOptions>) -> Result<String, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "predictSafeAddress",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn encode_deploy_safe(&self, args: &SafeArgsEncodeDeploySafe, invoke_options: Option<InvokeOptions>) -> Result<String, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "encodeDeploySafe",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn safe_is_deployed(&self, args: &SafeArgsSafeIsDeployed, invoke_options: Option<InvokeOptions>) -> Result<bool, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "safeIsDeployed",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn get_safe_initializer(&self, args: &SafeArgsGetSafeInitializer, invoke_options: Option<InvokeOptions>) -> Result<String, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "getSafeInitializer",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn create_transaction(&self, args: &SafeArgsCreateTransaction, invoke_options: Option<InvokeOptions>) -> Result<SafeSafeTransaction, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "createTransaction",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn create_multi_send_transaction(&self, args: &SafeArgsCreateMultiSendTransaction, invoke_options: Option<InvokeOptions>) -> Result<SafeSafeTransaction, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "createMultiSendTransaction",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn add_signature(&self, args: &SafeArgsAddSignature, invoke_options: Option<InvokeOptions>) -> Result<SafeSafeTransaction, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "addSignature",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn get_transaction_hash(&self, args: &SafeArgsGetTransactionHash, invoke_options: Option<InvokeOptions>) -> Result<String, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "getTransactionHash",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn sign_transaction_hash(&self, args: &SafeArgsSignTransactionHash, invoke_options: Option<InvokeOptions>) -> Result<SafeSafeSignature, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "signTransactionHash",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn approve_transaction_hash(&self, args: &SafeArgsApproveTransactionHash, invoke_options: Option<InvokeOptions>) -> Result<SafeEthersTxReceipt, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "approveTransactionHash",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn execute_transaction(&self, args: &SafeArgsExecuteTransaction, invoke_options: Option<InvokeOptions>) -> Result<SafeEthersTxReceipt, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "executeTransaction",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn get_owners_who_approved_tx(&self, args: &SafeArgsGetOwnersWhoApprovedTx, invoke_options: Option<InvokeOptions>) -> Result<Vec<String>, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "getOwnersWhoApprovedTx",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn get_signature(&self, args: &SafeArgsGetSignature, invoke_options: Option<InvokeOptions>) -> Result<SafeSafeTransaction, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "getSignature",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn get_owners(&self, args: &SafeArgsGetOwners, invoke_options: Option<InvokeOptions>) -> Result<Vec<String>, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "getOwners",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn get_threshold(&self, args: &SafeArgsGetThreshold, invoke_options: Option<InvokeOptions>) -> Result<u32, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "getThreshold",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn is_owner(&self, args: &SafeArgsIsOwner, invoke_options: Option<InvokeOptions>) -> Result<bool, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "isOwner",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn get_modules(&self, args: &SafeArgsGetModules, invoke_options: Option<InvokeOptions>) -> Result<Vec<String>, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "getModules",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn is_module_enabled(&self, args: &SafeArgsIsModuleEnabled, invoke_options: Option<InvokeOptions>) -> Result<bool, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "isModuleEnabled",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn encode_enable_module_data(&self, args: &SafeArgsEncodeEnableModuleData, invoke_options: Option<InvokeOptions>) -> Result<String, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "encodeEnableModuleData",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn encode_disable_module_data(&self, args: &SafeArgsEncodeDisableModuleData, invoke_options: Option<InvokeOptions>) -> Result<String, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "encodeDisableModuleData",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn encode_multi_send_data(&self, args: &SafeArgsEncodeMultiSendData, invoke_options: Option<InvokeOptions>) -> Result<String, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "encodeMultiSendData",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn encode_add_owner_with_threshold_data(&self, args: &SafeArgsEncodeAddOwnerWithThresholdData, invoke_options: Option<InvokeOptions>) -> Result<String, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "encodeAddOwnerWithThresholdData",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn encode_remove_owner_data(&self, args: &SafeArgsEncodeRemoveOwnerData, invoke_options: Option<InvokeOptions>) -> Result<String, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "encodeRemoveOwnerData",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn encode_swap_owner_data(&self, args: &SafeArgsEncodeSwapOwnerData, invoke_options: Option<InvokeOptions>) -> Result<String, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "encodeSwapOwnerData",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn encode_change_threshold_data(&self, args: &SafeArgsEncodeChangeThresholdData, invoke_options: Option<InvokeOptions>) -> Result<String, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "encodeChangeThresholdData",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }
}
// Imported Modules END //
