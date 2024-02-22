#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[pink::contract]
mod start_sidevm {
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
        pub fn start(&self) {
            if self.env().caller() != self.owner {
                return;
            }
            let hash = *include_bytes!("./sideprog.wasm.hash");
            pink::start_sidevm(hash).expect("Failed to start sidevm");
        }
    }
}
