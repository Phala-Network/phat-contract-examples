#![cfg_attr(not(feature = "std"), no_std)]

use pink_extension as pink;

#[pink::contract]
mod start_sidevm {
    use super::pink;

    #[ink(storage)]
    pub struct Contract {}

    impl Contract {
        #[ink(constructor)]
        pub fn default() -> Self {
            let hash = *include_bytes!("./sideprog.wasm.hash");
            pink::start_sidevm(hash, true);
            Self {}
        }
        #[ink(message)]
        pub fn test(&self) {
        }
    }
}
