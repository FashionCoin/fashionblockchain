use exonum::{
    api::{self, ServiceApiBuilder, ServiceApiState},
    crypto::{Hash, PublicKey},
};

//use transactions::NodeTransactions;
use structs::{Wallet, TransferRecord};
use Schema;
use transactions::NodeTransactions;
use exonum::blockchain::Transaction;
use exonum::node::TransactionSend;

/// Describes the query parameters for the `get_wallet` endpoint.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct WalletQuery {
    /// Public key of the queried wallet.
    pub pub_key: PublicKey,
}

/// Response to an incoming transaction returned by the REST API.
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionResponse {
    /// Hash of the transaction.
    pub tx_hash: Hash,
}

/// Wallet history.
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletHistory {
    /// List of above transactions.
    pub result: Vec<TransferRecord>,
}

/// Wallet information.
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletInfo {
    /// Proof of the last block.
    pub result: Option<Wallet>,
}

/// Public service API description.
#[derive(Debug, Clone, Copy)]
pub struct PublicApi;

impl PublicApi {
    pub fn get_wallet_transfers(state: &ServiceApiState, query: WalletQuery) -> api::Result<WalletHistory> {
        let public_key = &query.pub_key;

        let snapshot = state.snapshot();
        let schema = Schema::new(&snapshot);

        let wallet = schema.wallet(public_key);
        let mut result = Vec::new();

        if Some(&wallet).is_some() {
            let some_wallet = schema.wallet(public_key);
            let wallet = some_wallet.unwrap();
            let mut some_transfer = schema.transfer(wallet.ref_prev());
            loop {
                if some_transfer.is_some() {
                    let transfer = some_transfer.unwrap();
                    let clone = transfer.clone();
                    let temp;
                    if *transfer.from() == *public_key {
                        temp = transfer.ref_from();
                    } else {
                        temp = transfer.ref_to();
                    }

                    if *temp == Hash::zero() {
                        result.push(clone);
                        break;
                    } else {
                        result.push(clone);
                        some_transfer = schema.transfer(temp);
                    }
                } else {
                    break;
                }
            }
        }
        Ok(WalletHistory {
            result: result,
        })
    }

    pub fn post_transaction(state: &ServiceApiState, query: NodeTransactions) -> api::Result<TransactionResponse> {
        let transaction: Box<Transaction> = query.into();
        let tx_hash = transaction.hash();
        state.sender().send(transaction)?;
        Ok(TransactionResponse { tx_hash })
    }


    /// NEW ///
    /// Endpoint for getting a single wallet.
    pub fn wallet_info(state: &ServiceApiState, query: WalletQuery) -> api::Result<WalletInfo> {
        let public_key = &query.pub_key;

        let snapshot = state.snapshot();
        let currency_schema = Schema::new(&snapshot);

        let wallet = currency_schema.wallet(public_key);

        Ok(WalletInfo {
            result: wallet,
        })
    }

    /// Wires the above endpoint to public scope of the given `ServiceApiBuilder`.
    pub fn wire(builder: &mut ServiceApiBuilder) {
        builder
            .public_scope()
            .endpoint("v1/wallets/info", Self::wallet_info)
            .endpoint("v1/wallets/history", Self::get_wallet_transfers)
            .endpoint_mut("v1/wallets/transaction", Self::post_transaction);
    }
}