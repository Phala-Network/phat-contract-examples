#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

#[ink::contract]
mod web3 {
    use alloc::string::String;

    use pink_web3::api::{Accounts, Eth, Namespace};
    use pink_web3::keys::pink::KeyPair;
    use pink_web3::transports::{resolve_ready, PinkHttp};
    use pink_web3::types::TransactionParameters;

    #[ink(storage)]
    pub struct Web3 {
        url: String,
    }

    impl Web3 {
        fn eth(&self) -> Eth<PinkHttp> {
            Eth::new(PinkHttp::new(self.url.clone()))
        }

        fn accounts(&self) -> Accounts<PinkHttp> {
            Accounts::new(PinkHttp::new(self.url.clone()))
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self {
                url: "http://localhost:3333".into(),
            }
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

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn it_works() {
            pink_extension_runtime::mock_ext::mock_all_ext();
            let web3 = Web3::default();
            assert_eq!(web3.mining(), true);
        }

        #[ink::test]
        fn tx_works() {
            pink_extension_runtime::mock_ext::mock_all_ext();
            let web3 = Web3::default();
            _ = web3.tx();
        }
    }
}
