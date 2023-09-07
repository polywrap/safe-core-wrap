extern crate safe_core_wrap_playground;
use safe_core_wrap_playground::{
    get_client, Ethers, EthersArgsGetSignerAddress, InvokeOptions, Safe, SafeArgsDeploySafe,
    SafeDeploymentInput, SafeEthersConnection, SafeSafeAccountConfig as SafeAccountConfig,
    SafeSafeDeploymentConfig as SafeDeploymentConfig, NETWORK, SALT_NONCE,
};

pub fn main() {
    let client = get_client();

    let invoke_options = InvokeOptions {
        uri: None,
        client: Some(client),
        env: None,
    };

    let ethers = Ethers::new(Some(invoke_options.clone()));
    let safe = Safe::new(Some(invoke_options.clone()));

    let signer_address =
        ethers.get_signer_address(&EthersArgsGetSignerAddress { connection: None }, None);

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
        &SafeArgsDeploySafe {
            input: deployment_input,
            tx_options: None,
        },
        None,
    );

    println!("Safe deployed to the address: {}", deploy.unwrap());
}
