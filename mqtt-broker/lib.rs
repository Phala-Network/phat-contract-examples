#![cfg_attr(not(feature = "std"), no_std)]

use pink_extension as pink;

#[pink::contract]
mod start_sidevm {
    use super::pink;

    #[ink(storage)]
    pub struct Contract {
        owner: AccountId,
    }

    impl Contract {
        #[ink(constructor)]
        pub fn default() -> Self {
            Self {
                owner: Self::env().caller(),
            }
        }

        #[ink(message)]
        pub fn start(&self) -> bool {
            if self.env().caller() != self.owner {
                return false;
            }
            let hash = *include_bytes!("./sideprog.wasm.hash");
            pink::start_sidevm(hash).expect("Failed to start sidevm");
            true
        }
    }
}
