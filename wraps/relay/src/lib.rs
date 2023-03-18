pub mod wrap;
use polywrap_wasm_rs::BigInt;
pub use wrap::*;

use relay_adapter::{GelatoRelayer, RelayAdapter};
pub mod relay_adapter;

pub fn relay_transaction(args: ArgsRelayTransaction, env: Option<Env>) -> RelayResponse {
    GelatoRelayer::relay_transaction(args.transaction, env)
}

pub fn get_estimate_fee(args: ArgsGetEstimateFee) -> BigInt {
    let chain_id = u64::from_str_radix(args.chain_id.to_string().as_str(), 10).unwrap();
    let gas_limit = u64::from_str_radix(args.gas_limit.to_string().as_str(), 10).unwrap();

    GelatoRelayer::get_estimate_fee(chain_id, gas_limit, args.gas_token)
}

pub fn get_fee_collector(_: ArgsGetFeeCollector) -> String {
    GelatoRelayer::get_fee_collector()
}
