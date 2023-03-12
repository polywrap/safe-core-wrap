pub mod wrap;
pub use wrap::*;

use account_abstraction::{Safe, AccountAbstraction};


pub mod account_abstraction;

pub fn relay_transaction(args: ArgsRelayTransaction, env: Env) -> String {
    if let None = env.connection {
        panic!("No connection given through env")
    }
    let safe = Safe::new(env.connection.unwrap(), None);
    let relayed_tx_hash = safe.relay_transaction(args.transaction, args.options);
    relayed_tx_hash
}