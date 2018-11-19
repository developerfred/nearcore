use jsonrpc_core::{IoHandler, Result as JsonRpcResult};

use client::Client;
use primitives::types::{SignedTransaction, TransactionBody, ViewCall, ViewCallResult};

build_rpc_trait! {
    pub trait TransactionApi {
        /// Receive new transaction.
        #[rpc(name = "receive_transaction")]
        fn rpc_receive_transaction(&self, TransactionBody) -> JsonRpcResult<()>;
        /// Call view function.
        #[rpc(name = "view")]
        fn rpc_view(&self, ViewCall) -> JsonRpcResult<ViewCallResult>;
    }
}

impl TransactionApi for Client {
    fn rpc_receive_transaction(&self, t: TransactionBody) -> JsonRpcResult<()> {
        let transaction = SignedTransaction::new(123, t);
        Ok(self.receive_transaction(transaction))
    }
    fn rpc_view(&self, v: ViewCall) -> JsonRpcResult<ViewCallResult> {
        Ok(self.view_call(&v))
    }
}

pub fn get_handler(client: Client) -> IoHandler {
    let mut io = IoHandler::new();
    io.extend_with(client.to_delegate());
    io
}

#[cfg(test)]
mod tests {
    use super::*;

    extern crate jsonrpc_test;
    extern crate primitives;
    extern crate storage;

    use self::jsonrpc_test::Rpc;
    use primitives::types::{SignedTransaction, TransactionBody};

    #[test]
    fn test_call() {
        let storage = storage::Storage::new("storage/db-test");
        let handler = get_handler(Client::new(storage));
        let rpc = Rpc::from(handler);
        let t = TransactionBody {
            nonce: 0,
            sender: 1,
            receiver: 0,
            amount: 0,
        };
        assert_eq!(rpc.request("receive_transaction", &[t]), "null");
    }
}