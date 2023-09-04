extern crate safe_core_wrap_playground;
use safe_core_wrap_playground::{
    get_client, EthersModule, EthersModuleArgsGetSignerAddress, SafeDeploymentInput,
    SafeEthersConnection, SafeModule, SafeModuleArgsDeploySafe,
    SafeSafeAccountConfig as SafeAccountConfig, SafeSafeDeploymentConfig as SafeDeploymentConfig,
    NETWORK, SALT_NONCE,
};

pub fn main() {
    let client = get_client();

    let ethers = EthersModule::new(None, Some(client.clone()), None);
    let safe = SafeModule::new(None, Some(client.clone()), None);

    let signer_address = ethers.get_signer_address(
        &EthersModuleArgsGetSignerAddress { connection: None },
        None,
        None,
        None,
    );

    println!(
        "Signer address fetched: {}",
        signer_address.clone().unwrap()
    );
    let deployment_input = SafeDeploymentInput {
        safe_account_config: SafeAccountConfig {
            owners: vec![signer_address.unwrap()],
            threshold: 1,
            to: None,
            data: None,
            fallback_handler: None,
            payment_token: None,
            payment: None,
            payment_receiver: None,
        },
        safe_deployment_config: Some(SafeDeploymentConfig {
            salt_nonce: SALT_NONCE.to_string(),
            is_l1_safe: None,
            version: None,
        }),
        connection: Some(SafeEthersConnection {
            network_name_or_chain_id: Some(NETWORK.clone().to_string()),
            node: None,
        }),
        custom_contract_addresses: None,
    };
    let deploy = safe.deploy_safe(
        &SafeModuleArgsDeploySafe {
            input: deployment_input,
            tx_options: None,
        },
        None,
        None,
        None,
    );

    println!("Safe deployed to the address: {}", deploy.unwrap());
}
