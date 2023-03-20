pub mod wrap;
pub use wrap::*;

use account_abstraction::{AccountAbstraction, Safe};

pub mod account_abstraction;

pub fn relay_transaction(args: ArgsRelayTransaction, env: Env) -> String {
    if let None = env.connection {
        panic!("No connection given through env")
    }
    let safe = Safe::new(env.connection.unwrap(), args.config);
    let relayed_tx_hash = safe.relay_transaction(args.transaction, args.options);
    relayed_tx_hash
}

pub fn get_safe_address(args: ArgsGetSafeAddress, env: Env) -> String {
    if let None = env.connection {
        panic!("No connection given through env")
    }
    let safe = Safe::new(env.connection.unwrap(), args.config);
    safe.get_address()
}
