pub mod wrap;
pub use wrap::*;
use polywrap_wasm_rs::BigInt;

pub fn relay_transaction(args: ArgsRelayTransaction) -> RelayResponse {
    return RelayResponse {
        task_id: String::from("foo")
    };
}

pub fn get_estimate_fee(args: ArgsGetEstimateFee) -> BigInt {
    BigInt::from(0)
}

pub fn get_fee_collector(_: ArgsGetFeeCollector) -> String {
    String::from("foo")
}