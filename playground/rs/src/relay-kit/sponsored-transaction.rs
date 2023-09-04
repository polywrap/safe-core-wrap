extern crate num_bigint;
extern crate safe_core_wrap_playground;

use std::ops::Add;

use num_bigint::BigInt;
use safe_core_wrap_playground::{
    create_transaction, get_client, AccountAbstractionDeploymentParameters,
    AccountAbstractionMetaTransactionOptions, AccountAbstractionModule,
    AccountAbstractionModuleArgsGetSafeAddress, AccountAbstractionModuleArgsRelayTransaction,
    EthersModule, RelayerModule, RelayerModuleArgsGetEstimateFee, SALT_NONCE,
};

pub fn main() {
    let client = get_client();

    let ethers = EthersModule::new(None, Some(client.clone()), None);
    let relay = RelayerModule::new(None, Some(client.clone()), None);
    let account_abstraction = AccountAbstractionModule::new(None, Some(client.clone()), None);
    let (transaction, gas_limit) = create_transaction(ethers.clone());

    let safe_address = account_abstraction.get_safe_address(
        &AccountAbstractionModuleArgsGetSafeAddress {
            config: Some(AccountAbstractionDeploymentParameters {
                salt_nonce: Some(SALT_NONCE.to_string()),
                custom_contract_addresses: None,
            }),
        },
        None,
        None,
        None,
    );

    println!("Predicted safe address: {}", safe_address.clone().unwrap());

    let estimation = relay.get_estimate_fee(
        &RelayerModuleArgsGetEstimateFee {
            chain_id: 5,
            gas_limit,
            gas_token: None,
        },
        None,
        None,
        None,
    );

    if estimation.is_err() {
        panic!(
            "Error fetching transaction estimation from relayer: {}",
            estimation.unwrap_err()
        );
    }
    let estimation_value: BigInt = estimation.unwrap().parse().unwrap();

    let gas_limit_with_buffer = estimation_value.add(BigInt::from(150000));
    let meta_transaction_options = AccountAbstractionMetaTransactionOptions {
        gas_limit: gas_limit_with_buffer.to_string(),
        gas_token: None,
        is_sponsored: Some(true),
    };

    let response = account_abstraction.relay_transaction(
        &AccountAbstractionModuleArgsRelayTransaction {
            transaction,
            options: meta_transaction_options,
            config: Some(AccountAbstractionDeploymentParameters {
                salt_nonce: Some(SALT_NONCE.to_string()),
                custom_contract_addresses: None,
            }),
        },
        None,
        None,
        None,
    );
    if response.is_err() {
        panic!("Error relaying the transaction: {}", response.unwrap_err());
    }
    println!("Transaction has been relayed...");
    println!(
        "Task URL: https://relay.gelato.digital/tasks/status/{}",
        response.unwrap()
    );
}
