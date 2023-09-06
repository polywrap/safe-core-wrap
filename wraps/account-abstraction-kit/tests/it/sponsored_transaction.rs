use std::ops::Add;

use polywrap_msgpack_serde::BigInt;

use super::{
    get_client,
    wrap::types::{
        AccountAbstractionDeploymentParameters, AccountAbstractionMetaTransactionData,
        AccountAbstractionMetaTransactionOptions, AccountAbstractionModule,
        AccountAbstractionModuleArgsRelayTransaction, EthersModule, EthersModuleArgsEncodeFunction,
        EthersModuleArgsEstimateTransactionGas, EthersTxRequest, RelayerModule,
        RelayerModuleArgsGetEstimateFee,
    },
    SALT_NONCE,
};

#[test]
fn paid_transaction() {
    let client = get_client();

    let ethers = EthersModule::new(None, Some(client.clone()), None);
    let account_abstraction = AccountAbstractionModule::new(None, Some(client.clone()), None);
    let relay = RelayerModule::new(None, Some(client.clone()), None);

    let encode_function = ethers.encode_function(
        &EthersModuleArgsEncodeFunction {
            method: "function store(uint256 num) public".to_string(),
            args: Some(vec!["9227".to_string()]),
        },
        None,
        None,
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
        &EthersModuleArgsEstimateTransactionGas {
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
        None,
        None,
    );

    let gas_limit_with_buffer: BigInt = gas_limit.unwrap().parse::<BigInt>().unwrap().add(150000);

    let estimation = relay.get_estimate_fee(
        &RelayerModuleArgsGetEstimateFee {
            chain_id: 5,
            gas_limit: gas_limit_with_buffer.to_string(),
            gas_token: None,
        },
        None,
        None,
        None,
    );

    let estimation_value: BigInt = estimation.unwrap().parse().unwrap();

    let gas_limit_with_buffer = estimation_value.add(BigInt::from(150000));
    let meta_transaction_options = AccountAbstractionMetaTransactionOptions {
        gas_limit: gas_limit_with_buffer.to_string(),
        gas_token: None,
        is_sponsored: Some(true),
    };

    let response = account_abstraction.relay_transaction(
        &AccountAbstractionModuleArgsRelayTransaction {
            transaction: meta_transaction,
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
    assert!(response.is_ok());
}
