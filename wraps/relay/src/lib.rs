pub mod wrap;
use polywrap_wasm_rs::BigInt;
pub use wrap::*;

use relay_adapter::{GelatoRelayer, RelayAdapter};
pub mod relay_adapter;

impl ModuleTrait for Module {
    fn relay_transaction(
        args: ArgsRelayTransaction,
        env: Option<Env>,
    ) -> Result<RelayResponse, String> {
        Ok(GelatoRelayer::relay_transaction(args.transaction, env))
    }

    fn get_estimate_fee(args: ArgsGetEstimateFee) -> Result<BigInt, String> {
        let chain_id = u64::from_str_radix(args.chain_id.to_string().as_str(), 10).unwrap();
        let gas_limit = u64::from_str_radix(args.gas_limit.to_string().as_str(), 10).unwrap();

        Ok(GelatoRelayer::get_estimate_fee(
            chain_id,
            gas_limit,
            args.gas_token,
        ))
    }

    fn get_fee_collector(_: ArgsGetFeeCollector) -> Result<String, String> {
        Ok(GelatoRelayer::get_fee_collector())
    }
}
