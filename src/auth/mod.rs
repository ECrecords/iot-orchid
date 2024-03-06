// Declare the submodules of the `auth` module
pub mod jwt;
pub mod error;

// Optionally re-export items to simplify their paths when used externally
pub use jwt::*;
pub use error::{Error, Result}; // This makes Error and Result accessible via `crate::auth::Error` and `crate::auth::Result`
