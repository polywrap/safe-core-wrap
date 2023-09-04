pub mod wrap;
pub use wrap::*;

use account_abstraction::{AccountAbstraction, Safe};

pub mod account_abstraction;

impl ModuleTrait for Module {
    fn relay_transaction(args: ArgsRelayTransaction, env: Env) -> Result<String, String> {
        if let None = env.connection {
            panic!("No connection given through env")
        }
        let safe = Safe::new(env.connection.unwrap(), args.config);
        let relayed_tx_hash = safe.relay_transaction(args.transaction, args.options);
        Ok(relayed_tx_hash)
    }

    fn get_safe_address(args: ArgsGetSafeAddress, env: Env) -> Result<String, String> {
        if let None = env.connection {
            panic!("No connection given through env")
        }
        let safe = Safe::new(env.connection.unwrap(), args.config);
        Ok(safe.get_address())
    }
}
