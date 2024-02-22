#![cfg_attr(not(feature = "std"), no_std, no_main)]
extern crate alloc;

#[pink::contract(env=PinkEnvironment)]
mod signing {
    use pink::chain_extension::signing as sig;
    use pink::PinkEnvironment;

    #[ink(storage)]
    pub struct Signing {}

    impl Signing {
        #[ink(constructor)]
        pub fn default() -> Self {
            Self {}
        }

        #[ink(message)]
        pub fn test(&self) {
            use sig::SigType;

            let privkey = sig::derive_sr25519_key(b"a spoon of salt");
            let pubkey = sig::get_public_key(&privkey, SigType::Sr25519);
            let message = b"hello world";
            let signature = sig::sign(message, &privkey, SigType::Sr25519);
            let pass = sig::verify(message, &pubkey, &signature, SigType::Sr25519);
            assert!(pass);
            let pass = sig::verify(b"Fake", &pubkey, &signature, SigType::Sr25519);
            assert!(!pass);
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn it_works() {
            pink_chain_extension::mock_ext::mock_all_ext();

            let contract = Signing::default();
            contract.test();
        }
    }
}
