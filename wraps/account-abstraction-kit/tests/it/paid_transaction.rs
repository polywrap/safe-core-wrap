use std::{ops::Add, str::FromStr};

use polywrap_msgpack_serde::BigInt;

use super::{
    get_client,
    wrap::types::{
        AccountAbstraction, AccountAbstractionArgsGetSafeAddress,
        AccountAbstractionArgsRelayTransaction, AccountAbstractionDeploymentParameters,
        AccountAbstractionMetaTransactionData, AccountAbstractionMetaTransactionOptions, Ethers,
        EthersArgsEncodeFunction, EthersArgsEstimateTransactionGas, EthersArgsGetBalance,
        EthersArgsSendTransactionAndWait, EthersArgsToEth, EthersTxRequest, InvokeOptions, Relayer,
        RelayerArgsGetEstimateFee,
    },
    SALT_NONCE,
};

#[test]
fn paid_transaction() {
    let client = get_client();

    let invoke_options = InvokeOptions {
        uri: None,
        client: Some(client),
        env: None,
    };
    let ethers = Ethers::new(Some(invoke_options.clone()));
    let account_abstraction = AccountAbstraction::new(Some(invoke_options.clone()));
    let relay = Relayer::new(Some(invoke_options.clone()));

    let encode_function = ethers.encode_function(
        &EthersArgsEncodeFunction {
            method: "function store(uint256 num) public".to_string(),
            args: Some(vec!["987".to_string()]),
        },
        None,
    );

    if encode_function.is_err() {
        panic!("{}", encode_function.unwrap_err())
    }

    let meta_transaction = AccountAbstractionMetaTransactionData {
        to: "0x56535D1162011E54aa2F6B003d02Db171c17e41e".to_string(),
        value: "0".to_string(),
        data: encode_function.unwrap(),
        operation: None,
    };
    let gas_limit = ethers.estimate_transaction_gas(
        &EthersArgsEstimateTransactionGas {
            tx: EthersTxRequest {
                to: Some(meta_transaction.clone().to),
                from: None,
                data: Some(meta_transaction.clone().data),
                _type: None,
                chain_id: None,
                access_list: None,
                gas_limit: None,
                max_fee_per_gas: None,
                max_priority_fee_per_gas: None,
                gas_price: None,
                value: Some(meta_transaction.clone().value),
                nonce: None,
            },
            connection: None,
        },
        None,
    );

    let gas_limit_with_buffer: BigInt = gas_limit.unwrap().parse::<BigInt>().unwrap().add(150000);

    let estimation = relay.get_estimate_fee(
        &RelayerArgsGetEstimateFee {
            chain_id: 5,
            gas_limit: gas_limit_with_buffer.to_string(),
            gas_token: None,
        },
        None,
    );

    let safe_address = account_abstraction.get_safe_address(
        &AccountAbstractionArgsGetSafeAddress {
            config: Some(AccountAbstractionDeploymentParameters {
                salt_nonce: Some(SALT_NONCE.to_string()),
                custom_contract_addresses: None,
            }),
        },
        None,
    );

    let safe_balance = ethers.get_balance(
        &EthersArgsGetBalance {
            address: safe_address.clone().unwrap(),
            block_tag: None,
            connection: None,
        },
        None,
    );

    let estimation_value: BigInt = estimation.unwrap().parse().unwrap();
    let estimation_in_wei = estimation_value.clone() * BigInt::from(10u64.pow(8));

    let safe_balance = BigInt::from_str(&safe_balance.unwrap()).unwrap();
    if safe_balance.lt(&estimation_in_wei) {
        let value_in_eth = ethers.to_eth(
            &EthersArgsToEth {
                wei: estimation_in_wei.to_string(),
            },
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
            &EthersArgsSendTransactionAndWait {
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
        &AccountAbstractionArgsRelayTransaction {
            transaction: meta_transaction,
            options: meta_transaction_options,
            config: Some(AccountAbstractionDeploymentParameters {
                salt_nonce: Some(SALT_NONCE.to_string()),
                custom_contract_addresses: None,
            }),
        },
        None,
    );
    assert!(response.is_ok());
}
