#![cfg_attr(not(feature = "std"), no_std, no_main)]
extern crate alloc;

#[ink::contract]
mod logging {
    #[ink(storage)]
    pub struct Logging {}

    impl Logging {
        #[ink(constructor)]
        pub fn default() -> Self {
            pink::error!("instantiated");
            Self {}
        }

        #[ink(message)]
        pub fn test(&self) {
            pink::error!("a test message received");
            pink::warn!("test end");
        }
    }
    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn log_works() {
            env_logger::init();
            pink_chain_extension::mock_ext::mock_all_ext();

            Logging::default().test()
        }
    }
}
