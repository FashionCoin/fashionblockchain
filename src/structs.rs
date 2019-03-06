// // // // // // // // // // PERSISTENT DATA // // // // // // // // // //
use exonum::crypto::{PublicKey, Hash};
//use std::time::SystemTime;
use chrono::DateTime;
use chrono::Utc;

encoding_struct! {
    struct Wallet {
        pub_key:            &PublicKey,
        balance:            u64,
        ref_prev:           &Hash,
        name_hash:          &Hash
    }
}

encoding_struct! {
    struct TransferRecord {
        from:        &PublicKey,
        ref_from:    &Hash,
        to:          &PublicKey,
        ref_to:      &Hash,
        amount:      u64,
        time:        DateTime<Utc>,
    }
}

encoding_struct! {
    struct RootPublicKey {
        pub_key:            &PublicKey,
    }
}




impl RootPublicKey {
    pub fn update_root_public_key(self, pub_key: &PublicKey) -> Self {
        Self::new(pub_key)
    }
}