extern crate num_bigint;
extern crate safe_core_wrap_playground;

use std::{ops::Add, str::FromStr};

use num_bigint::BigInt;
use safe_core_wrap_playground::{
    create_transaction, get_client, AccountAbstractionDeploymentParameters,
    AccountAbstractionMetaTransactionOptions, AccountAbstractionModule,
    AccountAbstractionModuleArgsGetSafeAddress, AccountAbstractionModuleArgsRelayTransaction,
    EthersModule, EthersModuleArgsGetBalance, EthersModuleArgsSendTransactionAndWait,
    EthersModuleArgsToEth, EthersTxRequest, RelayerModule, RelayerModuleArgsGetEstimateFee,
    SALT_NONCE,
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

    let safe_balance = ethers.get_balance(
        &EthersModuleArgsGetBalance {
            address: safe_address.clone().unwrap(),
            block_tag: None,
            connection: None,
        },
        None,
        None,
        None,
    );

    if safe_balance.is_err() {
        panic!("Error fetching safe balance: {}", safe_balance.unwrap_err());
    }

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
    let estimation_in_wei = estimation_value.clone() * BigInt::from(10u64.pow(8));

    let safe_balance = BigInt::from_str(&safe_balance.unwrap()).unwrap();
    if safe_balance.lt(&estimation_in_wei) {
        let value_in_eth = ethers.to_eth(
            &EthersModuleArgsToEth {
                wei: estimation_in_wei.to_string(),
            },
            None,
            None,
            None,
        );
        if value_in_eth.is_err() {
            panic!(
                "Error converting estimation to ETH: {}",
                value_in_eth.unwrap_err()
            );
        }
        println!("Funding the safe with {} ETH", value_in_eth.unwrap());
        let fund_tx = ethers.send_transaction_and_wait(
            &EthersModuleArgsSendTransactionAndWait {
                tx: EthersTxRequest {
                    value: Some(estimation_in_wei.to_string()),
                    to: Some(safe_address.unwrap()),
                    from: None,
                    data: Some("0x".to_string()),
                    _type: None,
                    chain_id: None,
                    access_list: None,
                    gas_limit: None,
                    max_fee_per_gas: None,
                    max_priority_fee_per_gas: None,
                    gas_price: None,
                    nonce: None,
                },
                connection: None,
            },
            None,
            None,
            None,
        );
        if fund_tx.is_err() {
            panic!("Error funding the safe: {}", fund_tx.unwrap_err());
        }
    }

    let gas_limit_with_buffer = estimation_value.add(BigInt::from(150000));
    let meta_transaction_options = AccountAbstractionMetaTransactionOptions {
        gas_limit: gas_limit_with_buffer.to_string(),
        gas_token: None,
        is_sponsored: None,
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
