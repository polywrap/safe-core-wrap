use super::wrap::types::{
    InvokeOptions, Relayer, RelayerArgsRelayTransaction, RelayerEnv, RelayerMetaTransactionOptions,
    RelayerRelayTransaction,
};
use polywrap_msgpack_serde::to_vec;

#[test]
fn call_relay_transaction() {
    let options = RelayerMetaTransactionOptions {
        gas_limit: "0".to_string(),
        gas_token: None,
        is_sponsored: None,
    };

    let invoke_options = InvokeOptions {
        uri: None,
        client: None,
        env: Some(
            to_vec(&RelayerEnv {
                relayer_api_key: "AiaCshYRyAUzTNfZZb8LftJaAl2SS3I8YwhJJXc5J7A_".to_string(),
            })
            .unwrap(),
        ),
    };

    let relayer = Relayer::new(Some(invoke_options));

    let transaction = RelayerRelayTransaction {
        target: "0xA045eb75e78f4988d42c3cd201365bDD5D76D406".to_string(),
        encoded_transaction: "0xae53dcae000000000000000000000000eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee000000000000000000000000d8da6bf26964af9d7eed9e03e53415d37aa9604500000000000000000000000000000000000000000000000000038d7ea4c68000".to_string(),
        chain_id: 5,
        options
    };

    let result = relayer.relay_transaction(&RelayerArgsRelayTransaction { transaction }, None);

    assert!(result.is_ok());
}
