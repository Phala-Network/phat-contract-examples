#![cfg_attr(not(feature = "std"), no_std)]

use pink_extension as pink;

#[pink::contract]
mod start_sidevm {
    use super::pink;
    use pink::PinkEnvironment;
    use scale::Encode;

    #[ink(storage)]
    pub struct Contract {}

    impl Contract {
        #[ink(constructor)]
        pub fn default() -> Self {
            pink::set_hook(
                pink::HookPoint::OnBlockEnd,
                ink::env::account_id::<PinkEnvironment>(),
                0x01,
                1000000000,
            );
            Self {}
        }
        #[ink(message, selector = 0x01)]
        pub fn on_block_end(&self) {
            let number = self.env().block_number().encode();
            pink::ext().cache_set(b"block_number", &number).unwrap();
            pink::push_sidevm_message(b"hello".to_vec());
        }

        #[ink(message)]
        pub fn start_sidevm(&self) {
            // TODO: check permission if needed
            let hash = *include_bytes!("./sideprog.wasm.hash");
            pink::start_sidevm(hash).expect("start sidevm failed");
        }

        #[ink(message)]
        pub fn stop_sidevm(&self) {
            // TODO: check permission if needed
            pink::push_sidevm_message(b"stop".to_vec());
        }

        #[ink(message)]
        pub fn force_stop_sidevm(&self) {
            // TODO: check permission if needed
            pink::force_stop_sidevm();
        }

        #[ink(message)]
        pub fn test(&self) {}
    }
}
