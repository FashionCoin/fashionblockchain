#![deny(missing_debug_implementations, unsafe_code)]

#[macro_use]
extern crate exonum;
extern crate failure;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate exonum_time;
extern crate chrono;

pub use schema::Schema;

pub mod api;
pub mod schema;
pub mod transactions;
pub mod structs;

use exonum::{
    api::ServiceApiBuilder, blockchain::{self, Transaction, TransactionSet}, crypto::Hash,
    encoding::Error as EncodingError, helpers::fabric::{self, Context}, messages::RawTransaction,
    storage::Snapshot,
};

use transactions::NodeTransactions;

/// Unique service ID.
const CRYPTOCURRENCY_SERVICE_ID: u16 = 0;
/// Name of the service.
const SERVICE_NAME: &str = "coin";
/// Initial balance of the wallet.
const INITIAL_BALANCE: u64 = 0;
const AI_RIGHT_KEY_FOR_DB: &str = "03e657ae71e51be60a45b4bd20bcf79ff52f0c037ae6da0540a0e0066132b472";
const AI_LEFT_KEY_FOR_DB: &str = "72b4326106e0a04005dae67a030c2ff59ff7bc20bdb4450ae61be571ae57e603";

/// Exonum `Service` implementation.
#[derive(Default, Debug)]
pub struct Service;

impl blockchain::Service for Service {
    fn service_id(&self) -> u16 {
        CRYPTOCURRENCY_SERVICE_ID
    }

    fn service_name(&self) -> &str {
        SERVICE_NAME
    }

    fn state_hash(&self, view: &dyn Snapshot) -> Vec<Hash> {
        let schema = Schema::new(view);
        schema.state_hash()
    }

    fn tx_from_raw(&self, raw: RawTransaction) -> Result<Box<dyn Transaction>, EncodingError> {
        NodeTransactions::tx_from_raw(raw).map(Into::into)
    }

    fn wire_api(&self, builder: &mut ServiceApiBuilder) {
        api::PublicApi::wire(builder);
    }
}

/// A configuration service creator for the `NodeBuilder`.
#[derive(Debug)]
pub struct ServiceFactory;

impl fabric::ServiceFactory for ServiceFactory {
    fn service_name(&self) -> &str {
        SERVICE_NAME
    }

    fn make_service(&mut self, _: &Context) -> Box<dyn blockchain::Service> {
        Box::new(Service)
    }
}