#![cfg_attr(not(test), warn(unused_crate_dependencies))]

#[macro_use]
extern crate tracing;

mod signer;
pub use signer::TrezorSigner;

mod types;
pub use types::{DerivationType as HDPath, TrezorError};
