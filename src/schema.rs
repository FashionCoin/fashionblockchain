use exonum::{
    crypto::{Hash, PublicKey}, storage::{Fork, ProofListIndex, ProofMapIndex, Snapshot},
};
use exonum::encoding::serialize::FromHex;
use structs::Wallet;
use exonum_time::schema::TimeSchema;
use structs::RootPublicKey;
use structs::TransferRecord;
use chrono::DateTime;
use chrono::Utc;


/// Database schema for the cryptocurrency.
#[derive(Debug)]
pub struct Schema<T> {
    view: T,
}

impl<T> AsMut<T> for Schema<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.view
    }
}

impl<T> Schema<T>
    where
        T: AsRef<dyn Snapshot>,
{
    /// Creates a new schema from the database view.
    pub fn new(view: T) -> Self {
        Schema { view }
    }

    pub fn wallets(&self) -> ProofMapIndex<&Snapshot, PublicKey, Wallet> {
        ProofMapIndex::new("cryptocurrency.wallets", self.view.as_ref())
    }

    pub fn wallet(&self, pub_key: &PublicKey) -> Option<Wallet> {
        self.wallets().get(pub_key)
    }

    pub fn root_public_key(&self, key_for_db: &str) -> Option<RootPublicKey> {
        let p1 = PublicKey::from_hex(key_for_db).unwrap();
        ProofMapIndex::new("cryptocurrency.root_key", &self.view).get(&p1)
    }

    pub fn transfers(&self) -> ProofMapIndex<&Snapshot, Hash, TransferRecord> {
        ProofMapIndex::new("cryptocurrency.transfers", self.view.as_ref())
    }
    pub fn transfer(&self, hash: &Hash) -> Option<TransferRecord> {
        self.transfers().get(hash)
    }

    pub fn propose_data_by_config_hash(&self) -> ProofMapIndex<&T, Hash, Wallet> {
        ProofMapIndex::new("cryptocurrency.wallets", &self.view)
    }

    pub fn config_hash_by_ordinal(&self) -> ProofListIndex<&T, Hash> {
        ProofListIndex::new("cryptocurrency.wallets", &self.view)
    }

    /// Returns the state hash of cryptocurrency service.
    pub fn state_hash(&self) -> Vec<Hash> {
        vec![self.wallets().merkle_root()]
    }
}

/// Implementation of mutable methods.
impl<'a> Schema<&'a mut Fork> {
    pub fn wallets_mut(&mut self) -> ProofMapIndex<&mut Fork, PublicKey, Wallet> {
        ProofMapIndex::new("cryptocurrency.wallets", &mut self.view)
    }

    pub fn root_public_key_mut(&mut self) -> ProofMapIndex<&mut Fork, PublicKey, RootPublicKey> {
        ProofMapIndex::new("cryptocurrency.root_key", &mut self.view)
    }

    pub fn transfers_mut(&mut self) -> ProofMapIndex<&mut Fork, Hash, TransferRecord> {
        ProofMapIndex::new("cryptocurrency.transfers", &mut self.view)
    }

    pub fn get_time(&mut self) -> DateTime<Utc> {
        TimeSchema::new(self.view.as_ref()).time().get().unwrap()
    }
}