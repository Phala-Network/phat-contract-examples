#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use pink_extension as pink;

#[pink::contract]
mod contract {
    use super::pink;
    use alloc::string::String;

    use pink_web3::api::{Accounts, Eth, Namespace};
    use pink_web3::keys::pink::KeyPair;
    use pink_web3::transports::{resolve_ready, PinkHttp};
    use pink_web3::types::TransactionParameters;

    #[ink(storage)]
    pub struct Contract {
        url: String,
    }

    impl Contract {
        #[ink(constructor)]
        pub fn default() -> Self {
            Self {
                url: "http://localhost:3333".into(),
            }
        }


        #[ink(message)]
        pub fn start(&self) -> bool {
            let code_hash = *include_bytes!("./sideprog.wasm.hash");
            pink::start_sidevm(code_hash).expect("Failed to start sidevm");
            true
        }

        #[ink(message)]
        pub fn log_test(&self, msg: alloc::string::String) -> bool {
            pink::info!("__bin_id__:{}", msg);
            true
        }

        fn eth(&self) -> Eth<PinkHttp> {
            Eth::new(PinkHttp::new(self.url.clone()))
        }

        fn accounts(&self) -> Accounts<PinkHttp> {
            Accounts::new(PinkHttp::new(self.url.clone()))
        }

        #[ink(message)]
        pub fn mining(&self) -> bool {
            self.eth().mining().resolve().unwrap()
        }

        #[ink(message)]
        pub fn tx(&self) {
            let key = KeyPair::derive_keypair(b"pink-wallet");
            let tx = TransactionParameters::default();
            let signed = resolve_ready(self.accounts().sign_transaction(tx, &key)).unwrap();
            self.eth()
                .send_raw_transaction(signed.raw_transaction)
                .resolve()
                .unwrap();
        }
    }
}
