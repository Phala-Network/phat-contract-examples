#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

use pink_extension as pink;
use pink::PinkEnvironment;

#[pink::contract(env = PinkEnvironment)]
mod proxy {
    use super::*;

    #[ink(storage)]
    pub struct Proxy {}

    impl Proxy {
        #[ink(constructor)]
        pub fn default() -> Self {
            let my_address = ink::env::account_id::<PinkEnvironment>();
            pink::set_hook(pink::HookPoint::OnBlockEnd,  my_address, 0x01, 1000000000);
            Proxy {}
        }

        #[ink(message, selector = 0x01)]
        pub fn on_block_end(&self) {
            pink::info!("on block end");
        }
    }
}
