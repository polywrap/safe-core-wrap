extern crate polywrap;
extern crate polywrap_ethereum_wallet_plugin;
extern crate serde;

pub mod wrap;
pub use wrap::types::*;

use std::{collections::HashMap, env, sync::Arc};

use polywrap::*;
use polywrap_ethereum_wallet_plugin::{
    connection::Connection as WalletConnection, connections::Connections, EthereumWalletPlugin,
};

pub const NETWORK: &str = "goerli";
pub const SALT_NONCE: &str = "0x92291";

#[derive(Serialize)]
pub struct AccountAbstractionKitEnv {
    pub connection: EthersConnection,
}

pub fn get_client() -> Arc<PolywrapClient> {
    dotenv::from_path("../.env").ok();

    let mut config = PolywrapClientConfig::new();
    config.add(SystemClientConfig::default().into());
    let signer = env::var("PRIVATE_KEY").unwrap_or_else(|_| {
        "0x4f3edf983ac636a65a842ce7c78d9aa706d3b113bce9c46f30d7d21715b23b1d".to_string()
    });

    let url = env::var("RPC_URL").unwrap_or_else(|_| {
        "https://goerli.infura.io/v3/41fbecf847994df5a9652b1210effd8a".to_string()
    });

    let connection = WalletConnection::new(url, Some(signer)).unwrap();
    let connections = Connections::new(
        HashMap::from([(NETWORK.to_string(), connection)]),
        Some(NETWORK.to_string()),
    );

    let wallet_plugin = EthereumWalletPlugin::new(connections);
    let plugin_pkg: PluginPackage<EthereumWalletPlugin> = wallet_plugin.into();
    let ethers_wallet_package = Arc::new(plugin_pkg);

    config
        .add_package(
            uri!("wrapscan.io/polywrap/ethereum-wallet@1.0"),
            ethers_wallet_package,
        )
        .add_envs(HashMap::from([
            (
                uri!("wrapscan.io/polywrap/relay-kit@0.1.0"),
                to_vec(&RelayerEnv {
                    relayer_api_key: "AiaCshYRyAUzTNfZZb8LftJaAl2SS3I8YwhJJXc5J7A_".to_string(),
                })
                .unwrap(),
            ),
            (
                uri!("wrapscan.io/polywrap/account-abstraction-kit@0.1.0"),
                to_vec(&AccountAbstractionKitEnv {
                    connection: EthersConnection {
                        node: None,
                        network_name_or_chain_id: Some("goerli".to_string()),
                    },
                })
                .unwrap(),
            ),
        ]))
        .add_redirects(HashMap::from([
            (
                uri!("wrapscan.io/polywrap/account-abstraction-kit@0.1.0"),
                uri!("fs/../../wraps/account-abstraction-kit/build"),
            ),
            (
                uri!("wrapscan.io/polywrap/protocol-kit@0.1.0"),
                uri!("fs/../../wraps/protocol-kit/build"),
            ),
            (
                uri!("wrapscan.io/polywrap/relay-kit@0.1.0"),
                uri!("fs/../../wraps/relay-kit/build"),
            ),
        ]));
    let client = PolywrapClient::new(config.build());
    Arc::new(client)
}

pub fn create_transaction(ethers: Ethers) -> (AccountAbstractionMetaTransactionData, String) {
    let encoded_function = ethers.encode_function(
        &EthersArgsEncodeFunction {
            method: "function store(uint256 num) public".to_string(),
            args: Some(vec!["99".to_string()]),
        },
        None,
    );
    let transaction = AccountAbstractionMetaTransactionData {
        to: "0x56535D1162011E54aa2F6B003d02Db171c17e41e".to_string(),
        value: "0".to_string(),
        data: encoded_function.unwrap(),
        operation: Some(AccountAbstractionOperationType::Call),
    };

    let gas_limit = ethers.estimate_transaction_gas(
        &EthersArgsEstimateTransactionGas {
            tx: EthersTxRequest {
                to: Some(transaction.clone().to),
                from: None,
                data: Some(transaction.clone().data),
                _type: None,
                chain_id: None,
                access_list: None,
                gas_limit: None,
                max_fee_per_gas: None,
                max_priority_fee_per_gas: None,
                gas_price: None,
                value: None,
                nonce: None,
            },
            connection: None,
        },
        None,
    );
    (transaction, gas_limit.unwrap())
}
