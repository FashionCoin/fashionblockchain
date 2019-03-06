// // // // // // // // // // TRANSACTIONS // // // // // // // // // //

use exonum::blockchain::Transaction;
use exonum::storage::Fork;
use exonum::blockchain::ExecutionResult;
use exonum::blockchain::ExecutionError;
use {AI_RIGHT_KEY_FOR_DB, AI_LEFT_KEY_FOR_DB, CRYPTOCURRENCY_SERVICE_ID, INITIAL_BALANCE};
use Schema;
use structs::{Wallet, TransferRecord, RootPublicKey};
use exonum::crypto::{PublicKey, Hash};
use exonum::messages::Message;
use exonum::crypto::CryptoHash;
use exonum::encoding::serialize::FromHex;

transactions! {
     pub NodeTransactions {
         const SERVICE_ID = CRYPTOCURRENCY_SERVICE_ID;

         struct TxCreateWallet {
            pub_key:     &PublicKey,
         }

         struct TxTransfer {
            from:        &PublicKey,
            to:          &PublicKey,
            amount:      u64,
            seed:        u64,
         }

         struct TxMint {
            pub_key:     &PublicKey,
            amount:      u64,
            seed:        u64,
         }

         struct TxRootSet {
            left_right:  u64,
            pub_key:     &PublicKey,
            seed:        u64,
         }

         struct TxCryptoname {
            name_hash:   &Hash,
            pub_key:     &PublicKey,
            seed:        u64
         }

     }
}

// // // // // // // // // // CONTRACTS // // // // // // // // // //

impl Transaction for TxCreateWallet {
    fn verify(&self) -> bool {
        self.verify_signature(self.pub_key())
    }

    fn execute(&self, view: &mut Fork) -> ExecutionResult {
        let mut schema = Schema::new(view);
        if schema.wallet(self.pub_key()).is_none() {
            let wallet = Wallet::new(self.pub_key(), INITIAL_BALANCE, &Hash::zero(), &Hash::zero());
            println!("Create the wallet: {:?}", wallet);
            schema.wallets_mut().put(self.pub_key(), wallet);
            Ok(())
        } else {
            println!("Wallet already exist!");
            Err(ExecutionError::with_description(1, String::from("Wallet already exist")))
        }
    }
}

impl Transaction for TxTransfer {
    fn verify(&self) -> bool {
        (*self.from() != *self.to()) && self.verify_signature(self.from())
    }

    fn execute(&self, view: &mut Fork) -> ExecutionResult {
        let mut schema = Schema::new(view);
        let sender = schema.wallet(self.from());
        let receiver = schema.wallet(self.to());
        if let (Some(sender), Some(receiver)) = (sender, receiver) {
            let amount = self.amount();
            if sender.balance() >= amount {
                let time = schema.get_time();
                let transfer = TransferRecord::new(&self.from(), sender.ref_prev(), &self.to(), receiver.ref_prev(), amount, time);
                let hash = transfer.hash();
                schema.transfers_mut().put(&transfer.hash(), transfer);
//                 sender = sender.decrease(amount);
                let mut sender2 = Wallet::new(sender.pub_key(), sender.balance() - amount, &hash, sender.name_hash());
                let mut receiver2 = Wallet::new(receiver.pub_key(), receiver.balance() + amount, &hash, receiver.name_hash());
                println!("Transfer between wallets: {:?} => {:?}", sender2, receiver2);
                schema.wallets_mut().put(self.from(), sender2);
                schema.wallets_mut().put(self.to(), receiver2);
                Ok(())
            } else {
                println!("Outstanding balance!");
                Err(ExecutionError::with_description(2, String::from("Outstanding balance")))
            }
        } else {
            println!("Wallet not found");
            Err(ExecutionError::with_description(3, String::from("Wallet not found")))
        }
    }
}

impl Transaction for TxMint {
    fn verify(&self) -> bool {
        self.verify_signature(self.pub_key())
    }

    fn execute(&self, view: &mut Fork) -> ExecutionResult {
        let mut schema = Schema::new(view);
        let root_pub_key = schema.root_public_key(AI_RIGHT_KEY_FOR_DB);

        match root_pub_key {
            Some(x) => {
                if *x.pub_key() == *self.pub_key() {
                    let minter = schema.wallet(self.pub_key());

                    if let Some(minter) = minter {
                        let amount = self.amount();
                        let minter = Wallet::new(minter.pub_key(), minter.balance() + amount, minter.ref_prev(), minter.name_hash());
                        println!("Mint {} coins to wallet", amount);
                        let mut wallets = schema.wallets_mut();
                        wallets.put(self.pub_key(), minter);
                        Ok(())
                    }else{
                        Err(ExecutionError::with_description(4, String::from("Minter wallet not found")))
                    }
                } else {
                    Err(ExecutionError::with_description(4, String::from("AI key is wrong")))
                }
            }
            None => {
                Err(ExecutionError::with_description(4, String::from("Server Error. AI_RIGHT_KEY doesn't set")))
            }
        }
    }
}

impl Transaction for TxRootSet {
    fn verify(&self) -> bool {
        self.verify_signature(self.pub_key())
    }

    fn execute(&self, view: &mut Fork) -> ExecutionResult {
        let mut schema = Schema::new(view);

        let key_for_db = match self.left_right() {
            0 => AI_RIGHT_KEY_FOR_DB,
            1 => AI_LEFT_KEY_FOR_DB,
            _ => return Err(ExecutionError::with_description(4, String::from("Left Right parameter is wrong")))
        };


        let root_pub_key = schema.root_public_key(key_for_db);

        match root_pub_key {
            Some(_) => {
                    Err(ExecutionError::with_description(4, String::from("Error. Key is already set")))
            }
            None => {
                    let p1 = PublicKey::from_hex(key_for_db).unwrap();
                    let root_pub_key_new = RootPublicKey::new(self.pub_key());
                    schema.root_public_key_mut().put(&p1, root_pub_key_new);
                    Ok(())
            }
        }
    }
}


impl Transaction for TxCryptoname {
    fn verify(&self) -> bool {
        true
    }

    fn execute(&self, view: &mut Fork) -> ExecutionResult {
        let mut schema = Schema::new(view);
        let root_pub_key = schema.root_public_key(AI_LEFT_KEY_FOR_DB);

        match root_pub_key {
            Some(x) => {
                if self.verify_signature(x.pub_key()) {
                    // let cryptoname = Cryptoname::new(self.name_hash(), self.pub_key());
//                    println!("Create the cryptoname: {:?}", cryptoname);
                    // schema.cryptonames_mut().put(self.pub_key(), cryptoname);
                    let wallet = schema.wallet(self.pub_key());

                    match wallet {
                        Some(w) => {
                            let new_wallet = Wallet::new(self.pub_key(), w.balance(), w.ref_prev(), self.name_hash());
                            println!("Create the cryptoname: {:?}", new_wallet);
                            schema.wallets_mut().put(self.pub_key(), new_wallet);
                            Ok(())
                        }
                        None => {
                            println!("Wallet not found");
                            Err(ExecutionError::with_description(4, String::from("Wallet not found")))
                        }
                    }
                } else {
                    println!("The signature does not match the Root Key");
                    Err(ExecutionError::with_description(4, String::from("The signature does not match the Root Key")))
                }
            }
            None => {
                println!("Root Key not yet seted");
                Err(ExecutionError::with_description(4, String::from("Root Key not yet seted")))
            }
        }
    }
}
