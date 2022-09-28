#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use pink_extension as pink;

#[pink::contract(env = PinkEnvironment)]
mod sidevmop {
    use super::pink;
    use pink::system::Result;
    use pink::PinkEnvironment;

    #[ink(storage)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct SidevmOp {}

    impl SidevmOp {
        #[ink(constructor)]
        pub fn default() -> Self {
            Self {}
        }
    }

    impl pink::system::SidevmOperation for SidevmOp {
        #[ink(message)]
        fn deploy(&self, code_hash: pink::Hash) -> Result<()> {
            let system = pink::system::SystemRef::instance();
            system.deploy_sidevm_to(self.env().caller(), code_hash)
        }
    }
}
