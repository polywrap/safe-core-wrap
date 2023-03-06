pub mod wrap;
pub use wrap::*;

pub fn relay_transaction(args: ArgsRelayTransaction) -> RelayResponse {
    return RelayResponse {
        task_id: String::from("foo")
    };
}

