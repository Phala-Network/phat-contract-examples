#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod unittests {
    #[ink(storage)]
    pub struct Unittests {}

    impl Unittests {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        #[ink(message)]
        pub fn test(&self) {}
    }

    #[cfg(test)]
    mod tests {
        #[ink::test]
        fn getrandom_works() {
            pink_chain_extension::mock_ext::mock_all_ext();

            let bytes = pink::ext().getrandom(3);
            assert_eq!(bytes.len(), 3);
            assert!(bytes != [0; 3]);
        }
    }
}
