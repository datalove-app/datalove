//!

#![cfg_attr(not(feature = "std"), no_std)]

// mod keystore;
mod user;
#[cfg(feature = "std")]
mod vault;
mod wallet;

// pub use borsh;

// pub use error::Error;
pub use pubkey::Pubkey;
pub use wallet::*;

// /// Client interface for each underlying Vault keystore
// pub trait VaultDriver {
//     ///
//     fn init_sub_wallet() -> Result<[u8; 32], ()>;
// }

// /// Interface
// pub trait Persona {
//     ///
//     fn restore<const S: usize>(
//         derivation_path: &str,
//         wallet: wallet::WalletState<S>,
//     ) -> Result<(), ()>;
// }

#[cfg(feature = "std")]
mod pubkey {
    pub use solana_sdk::pubkey::Pubkey;

    pub const ZEROES: Pubkey = Pubkey::new_from_array([0u8; 32]);
}
#[cfg(not(feature = "std"))]
mod pubkey {
    pub const ZEROES: Pubkey = Pubkey::zeroes();

    #[derive(
        Clone,
        Copy,
        core::fmt::Debug,
        Default,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        bytemuck::Pod,
        bytemuck::Zeroable,
    )]
    #[repr(transparent)]
    pub struct Pubkey([u8; 32]);

    impl Pubkey {
        pub const fn zeroes() -> Self {
            Self([0u8; 32])
        }
    }

    impl From<[u8; 32]> for Pubkey {
        fn from(bytes: [u8; 32]) -> Self {
            Self(bytes)
        }
    }

    impl From<Pubkey> for [u8; 32] {
        fn from(key: Pubkey) -> Self {
            key.0
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
