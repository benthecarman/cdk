//! CDK common types and traits
pub mod amount;
pub mod dhke;
pub mod mint_url;
pub mod nuts;
pub mod secret;
pub mod util;
#[cfg(feature = "wallet")]
pub mod wallet;

pub use lightning_invoice::{self, Bolt11Invoice};

pub use self::amount::Amount;
pub use self::nuts::*;
pub use self::util::SECP256K1;

#[doc(hidden)]
#[macro_export]
macro_rules! ensure_cdk {
    ($cond:expr, $err:expr) => {
        if !$cond {
            return Err($err);
        }
    };
}
