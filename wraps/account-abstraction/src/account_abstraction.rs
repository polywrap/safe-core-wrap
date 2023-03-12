use polywrap_wasm_rs::BigInt;

use crate::{
    MetaTransactionData, 
    MetaTransactionOptions,
    SafeManagerSafeTransactionData as SafeTransactionData,
    SafeFactoryModule, 
    SafeContractsModule,
    imported::{
        safe_contracts_module::serialization::ArgsGetNonce,
        safe_contracts_ethereum_connection,
        safe_factory_module::serialization::ArgsPredictSafeAddress,
        safe_factory_ethereum_connection,
        safe_factory_safe_account_config::SafeFactorySafeAccountConfig, ArgsGetSignerAddress, SafeFactorySafeDeploymentConfig,
        ether_core_module::serialization::ArgsGetChainId
    },
    EtherCoreConnection,
    EtherCoreModule
};

// keccak256(toUtf8Bytes('Safe Account Abstraction'))
pub const PREDETERMINED_SALT_NONCE: &str = "0xb1073742015cbcf5a3a4d9d1ae33ecf619439710b89475f92e2abd2117e90f90";

pub trait AccountAbstraction {
    fn relay_transaction(
        &self,
        data: MetaTransactionData,
        options: MetaTransactionOptions
    ) -> String;
}

pub struct Safe {
    pub chain_id: u64,
    pub address: String,
    connection: EtherCoreConnection
}

impl AccountAbstraction for Safe {
    fn relay_transaction(
        &self,
        data: MetaTransactionData,
        options: MetaTransactionOptions
    ) -> String {
        let standardized_tx = self.standardize_transaction_data();

        // get signature from signer of the sanitized transaction data

        // encode the sanizited transaction and the signature

        // if safe is NOT deployed, we need to add the deployment
        // into the encoded transaction

        // if safe is deployed, just execute the previous sanitized transaction
        todo!()
    }
}

impl Safe {
    pub fn new(connection: EtherCoreConnection, salt_nonce: Option<String>) -> Self {
        let factory_connection = safe_factory_ethereum_connection::SafeFactoryEthereumConnection {
            node: connection.clone().node,
            network_name_or_chain_id: connection.clone().network_name_or_chain_id
        };

        let chain_id = EtherCoreModule::get_signer_address(&ArgsGetSignerAddress {
            connection: Some(connection.clone())
        }).unwrap();

        let signer = EtherCoreModule::get_chain_id(&ArgsGetChainId {
            connection: Some(connection.clone())
        }).unwrap();

        let safe_deployment_config: Option<SafeFactorySafeDeploymentConfig> = if let Some(salt) = salt_nonce {
            Some(SafeFactorySafeDeploymentConfig {
                salt_nonce: salt,
                is_l1_safe: None,
                version: None
            })
        } else {
            Some(SafeFactorySafeDeploymentConfig {
                salt_nonce: PREDETERMINED_SALT_NONCE.to_string(),
                is_l1_safe: None,
                version: None
            })
        };

        let predict_address_args = ArgsPredictSafeAddress {
            safe_account_config: SafeFactorySafeAccountConfig {
                owners: vec![signer],
                threshold: 1,
                fallback_handler: None,
                payment: None,
                payment_receiver: None,
                data: None,
                payment_token: None,
                to: None
            },
            safe_deployment_config,
            custom_contract_adressess: None,
            connection: Some(factory_connection)
        };
        let address = SafeFactoryModule::predict_safe_address(&predict_address_args).unwrap();
        Safe {
            chain_id: chain_id.parse::<u64>().unwrap(),
            address,
            connection
        }
    }

    pub fn get_address(&self) -> &String {
        &self.address
    }

    pub fn is_deployed(&self) -> bool {
        true
    }

    pub fn get_nonce(&self) -> BigInt {
        let connection = safe_contracts_ethereum_connection::SafeContractsEthereumConnection {
            node: self.connection.clone().node,
            network_name_or_chain_id: self.connection.clone().network_name_or_chain_id
        };
        SafeContractsModule::get_nonce(&ArgsGetNonce {
            address: self.address.as_str().to_string(),
            connection: Some(connection)
        }).unwrap()
    }

    pub fn get_signer_address(&self) -> String {
        "".to_string()
    }

    fn standardize_transaction_data(&self) -> SafeTransactionData {
        SafeTransactionData {
            data: "".to_string(),
            value: BigInt::from(0),
            to: "".to_string(),
            operation: None,
            safe_tx_gas: None,
            base_gas: None,
            gas_price: None,
            gas_token: None,
            refund_receiver: None,
            nonce: None,
        }
    }
}