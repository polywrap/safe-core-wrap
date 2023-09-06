use polywrap::*;
use polywrap_ethereum_wallet_plugin::{
    connection::Connection as WalletConnection, connections::Connections, EthereumWalletPlugin,
};
use serde::Serialize;
use std::{collections::HashMap, env, sync::Arc};

use wrap::types::EthersConnection;

mod paid_transaction;
mod sponsored_transaction;
mod wrap;

#[derive(Serialize)]
pub struct RelayKitEnv {
    #[serde(rename = "relayerApiKey")]
    pub relayer_api_key: String,
}

#[derive(Serialize)]
pub struct AccountAbstractionKitEnv {
    pub connection: EthersConnection,
}

pub const NETWORK: &str = "goerli";
pub const SALT_NONCE: &str = "0x32291";

pub fn get_client() -> Arc<PolywrapClient> {
    dotenv::dotenv().ok();
    let mut config = PolywrapClientConfig::new();

    config.add(SystemClientConfig::default().into());
    let signer = env::var("PRIVATE_KEY");
    if signer.is_err() {
        panic!("PRIVATE_KEY env variable is not set")
    };
    let url = env::var("RPC_URL").unwrap_or_else(|_| {
        "https://goerli.infura.io/v3/41fbecf847994df5a9652b1210effd8a".to_string()
    });

    let connection = WalletConnection::new(url, Some(signer.unwrap())).unwrap();
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
                uri!("wrapscan.io/polywrap/safe-relay-kit@0.0.1"),
                to_vec(&RelayKitEnv {
                    relayer_api_key: "AiaCshYRyAUzTNfZZb8LftJaAl2SS3I8YwhJJXc5J7A_".to_string(),
                })
                .unwrap(),
            ),
            (
                uri!("wrapscan.io/polywrap/safe-account-abstraction-kit@0.0.1"),
                to_vec(&AccountAbstractionKitEnv {
                    connection: EthersConnection {
                        node: None,
                        network_name_or_chain_id: Some(NETWORK.to_string()),
                    },
                })
                .unwrap(),
            ),
        ]))
        .add_redirects(HashMap::from([
            (
                uri!("wrapscan.io/polywrap/safe-account-abstraction-kit@0.0.1"),
                uri!("fs/../../wraps/account-abstraction-kit/build"),
            ),
            (
                uri!("wrapscan.io/polywrap/safe-protocol-kit@0.0.1"),
                uri!("fs/../../wraps/protocol-kit/build"),
            ),
            (
                uri!("wrapscan.io/polywrap/safe-relay-kit@0.0.1"),
                uri!("fs/../../wraps/relay-kit/build"),
            ),
        ]));
    Arc::new(PolywrapClient::new(config.build()))
}
