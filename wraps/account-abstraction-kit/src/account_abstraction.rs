use crate::wrap::{
    SafeSafeAccountConfig as SafeAccountConfig, SafeSafeDeploymentConfig as SafeDeploymentConfig,
    SafeSafeTransaction as SafeTransaction, SafeSafeTransactionData as SafeTransactionData,
    SafeSafeTransactionDataPartial as SafeTransactionDataPartial, *,
};
use polywrap_msgpack_serde::BigIntWrapper;
use polywrap_wasm_rs::BigInt;

pub const ZERO_ADDRESS: &str = "0x0000000000000000000000000000000000000000";

// keccak256(toUtf8Bytes('Safe Account Abstraction'))
pub const PREDETERMINED_SALT_NONCE: &str = "0x888";

pub trait AccountAbstraction {
    fn relay_transaction(
        &self,
        data: MetaTransactionData,
        options: MetaTransactionOptions,
    ) -> String;
}

#[derive(Clone)]
pub struct Safe {
    signer: String,
    chain_id: i32,
    address: String,
    factory_address: String,
    connection: EthersConnection,
    multi_send_call_only_address: String,
    deployment_info: SafeDeploymentInput,
}

impl AccountAbstraction for Safe {
    fn relay_transaction(
        &self,
        data: MetaTransactionData,
        options: MetaTransactionOptions,
    ) -> String {
        let standardized_tx = self.standardize_transaction_data(data.into(), options.clone());
        // get signature from signer of the sanitized transaction data
        let transaction = SafeModule::get_signature(&imported::ArgsGetSignature {
            tx: standardized_tx.into(),
            signing_method: None,
            safe_address: self.address.as_str().to_string(),
            connection: self.connection.clone().into(),
        })
        .unwrap();

        // encode the sanitized transaction w/signature
        let transaction_data =
            SafeModule::encode_exec_transaction(&imported::ArgsEncodeExecTransaction {
                safe_address: self.address.as_str().to_string(),
                safe_transaction: transaction.into(),
            })
            .unwrap();

        let (relay_transaction_target, encoded_transaction) = if self.is_deployed() {
            // if safe is deployed, just execute the previous sanitized transaction
            (self.address.as_str().to_string(), transaction_data)
        } else {
            // if safe is NOT deployed, we need to add the deployment
            let safe_deployment_transaction = SafeTransactionDataPartial {
                to: self.factory_address.as_str().to_string(),
                value: BigIntWrapper(BigInt::from(0)),
                data: SafeModule::encode_deploy_safe(&imported::ArgsEncodeDeploySafe {
                    input: self.deployment_info.clone(),
                })
                .unwrap(),
                operation: None,
                safe_tx_gas: None,
                base_gas: None,
                gas_price: None,
                gas_token: None,
                refund_receiver: None,
                nonce: None,
            };

            let safe_transaction = SafeTransactionDataPartial {
                to: self.address.as_str().to_string(),
                value: BigIntWrapper(BigInt::from(0)),
                data: transaction_data,
                operation: None,
                safe_tx_gas: None,
                base_gas: None,
                gas_price: None,
                gas_token: None,
                refund_receiver: None,
                nonce: None,
            };

            let multi_send_data =
                SafeModule::encode_multi_send_data(&imported::ArgsEncodeMultiSendData {
                    txs: vec![safe_deployment_transaction, safe_transaction],
                })
                .unwrap();

            (
                self.multi_send_call_only_address.as_str().to_string(),
                multi_send_data,
            )
        };

        let relay_transaction = RelayerRelayTransaction {
            chain_id: self.chain_id,
            encoded_transaction,
            target: relay_transaction_target,
            options: RelayerMetaTransactionOptions {
                gas_limit: options.gas_limit,
                gas_token: options.gas_token,
                is_sponsored: options.is_sponsored,
            },
        };
        let transaction_relayed =
            RelayerModule::relay_transaction(&imported::ArgsRelayTransaction {
                transaction: relay_transaction,
            })
            .unwrap();

        transaction_relayed.task_id
    }
}

impl Safe {
    pub fn new(connection: EthersConnection, params: Option<DeploymentParameters>) -> Self {
        let chain_id = EthersModule::get_chain_id(&imported::ArgsGetChainId {
            connection: Some(connection.clone()),
        })
        .unwrap();

        let signer = EthersModule::get_signer_address(&imported::ArgsGetSignerAddress {
            connection: Some(connection.clone()),
        })
        .unwrap();

        let mut custom_contract_addresses = None;
        let mut factory_address = String::new();
        let mut salt_nonce = PREDETERMINED_SALT_NONCE.to_string();

        if let Some(config) = params {
            if let Some(salt) = config.salt_nonce {
                salt_nonce = salt
            }

            if let Some(custom_contracts) = config.custom_contract_addresses {
                if let Some(factory_contract) = &custom_contracts.proxy_factory_contract {
                    factory_address = factory_contract.as_str().to_string();
                }
                custom_contract_addresses = Some(custom_contracts)
            }
        }

        let get_safe_contract_network_args = imported::ArgsGetSafeContractNetworks {
            chain_id: chain_id.as_str().to_string(),
            is_l1_safe: None,
            version: String::from("1.3.0"),
            filter: Some(SafeContractNetworksFilter {
                fallback_handler_address: false,
                multi_send_address: false,
                multi_send_call_only_address: true,
                safe_master_copy_address: false,
                safe_proxy_factory_address: factory_address.is_empty(),
            }),
        };

        let safe_contracts_networks =
            SafeModule::get_safe_contract_networks(&get_safe_contract_network_args).unwrap();

        if factory_address.is_empty() {
            if let Some(address) = safe_contracts_networks.safe_proxy_factory_address {
                factory_address = address;
            }
        };

        let multi_send_call_only_address =
            if let Some(address) = safe_contracts_networks.multi_send_call_only_address {
                address
            } else {
                panic!("Could not fetch multisend contract")
            };

        let deployment_config = SafeDeploymentConfig {
            salt_nonce,
            is_l1_safe: None,
            version: None,
        };
        let deployment_info = SafeDeploymentInput {
            safe_account_config: SafeAccountConfig {
                owners: vec![signer.as_str().to_string()],
                threshold: 1,
                fallback_handler: None,
                payment: None,
                payment_receiver: None,
                data: None,
                payment_token: None,
                to: None,
            },
            safe_deployment_config: Some(deployment_config),
            custom_contract_addresses,
            connection: Some(connection.clone().into()),
        };
        let predict_address_args = imported::ArgsPredictSafeAddress {
            input: deployment_info.clone(),
        };
        let address = SafeModule::predict_safe_address(&predict_address_args).unwrap();

        Safe {
            chain_id: chain_id.parse::<i32>().unwrap(),
            signer: signer.as_str().to_string(),
            address,
            factory_address,
            multi_send_call_only_address,
            connection,
            deployment_info,
        }
    }

    pub fn get_address(self) -> String {
        self.address
    }
    pub fn is_deployed(&self) -> bool {
        //@TODO: Check why this does not works
        // SafeFactoryModule::safe_is_deployed(&ArgsSafeIsDeployed {
        //     safe_address: self.address.as_str().to_string(),
        //     connection: safe_factory_ethereum_connection::SafeFactoryEthereumConnection {
        //         node: self.connection.clone().node,
        //         network_name_or_chain_id: self.connection.clone().network_name_or_chain_id,
        //     },
        // })
        // .unwrap()
        self.get_nonce().0 > BigInt::from(0)
    }

    pub fn get_nonce(&self) -> BigIntWrapper {
        let nonce = SafeModule::get_nonce(&imported::ArgsGetNonce {
            address: self.address.as_str().to_string(),
            connection: Some(self.connection.clone().into()),
        });

        if let Ok(n) = nonce {
            n
        } else {
            BigIntWrapper(BigInt::from(0))
        }
    }

    pub fn get_signer_address(&self) -> &String {
        &self.signer
    }

    fn standardize_transaction_data(
        &self,
        transaction: SafeMetaTransactionData,
        options: MetaTransactionOptions,
    ) -> SafeTransaction {
        let is_sponsored = if let Some(i) = options.is_sponsored {
            i
        } else {
            false
        };

        let gas_token = if let Some(token) = options.gas_token {
            Box::new(token)
        } else {
            Box::new(ZERO_ADDRESS.to_string())
        };

        let estimation = RelayerModule::get_estimate_fee(&imported::ArgsGetEstimateFee {
            chain_id: self.chain_id,
            gas_limit: options.gas_limit,
            gas_token: Some(*gas_token.clone()),
        })
        .unwrap();

        let base_gas = if is_sponsored {
            BigIntWrapper(BigInt::from(0))
        } else {
            estimation
        };

        let gas_price = if is_sponsored {
            BigIntWrapper(BigInt::from(0))
        } else {
            BigIntWrapper(BigInt::from(1))
        };

        let refund_receiver = if is_sponsored {
            ZERO_ADDRESS.to_string()
        } else {
            RelayerModule::get_fee_collector(&imported::ArgsGetFeeCollector {}).unwrap()
        };

        SafeTransaction {
            data: SafeTransactionData {
                data: transaction.data,
                value: transaction.value,
                to: transaction.to,
                operation: transaction.operation,
                safe_tx_gas: BigIntWrapper(BigInt::from(0)),
                base_gas: base_gas,
                gas_price: gas_price,
                gas_token: *gas_token,
                refund_receiver: refund_receiver,
                nonce: self.get_nonce().0.to_string().parse::<i32>().unwrap(),
            },
            signatures: None,
        }
    }
}

impl From<SafeEthersConnection> for EthersConnection {
    fn from(safe_connection: SafeEthersConnection) -> Self {
        EthersConnection {
            network_name_or_chain_id: safe_connection.network_name_or_chain_id,
            node: safe_connection.node,
        }
    }
}

impl From<EthersConnection> for SafeEthersConnection {
    fn from(safe_connection: EthersConnection) -> Self {
        SafeEthersConnection {
            network_name_or_chain_id: safe_connection.network_name_or_chain_id,
            node: safe_connection.node,
        }
    }
}

impl From<MetaTransactionData> for SafeMetaTransactionData {
    fn from(data: MetaTransactionData) -> Self {
        SafeMetaTransactionData {
            to: data.to,
            value: data.value,
            data: data.data,
            operation: Some(data.operation.into()),
        }
    }
}

impl From<Option<OperationType>> for SafeOperationType {
    fn from(operation: Option<OperationType>) -> Self {
        if let Some(op) = operation {
            match op {
                OperationType::Call => SafeOperationType::Call,
                OperationType::DelegateCall => SafeOperationType::DelegateCall,
                OperationType::_MAX_ => SafeOperationType::_MAX_,
            }
        } else {
            SafeOperationType::Call
        }
    }
}
