// Declare the submodules of the `auth` module
pub mod jwt;
pub mod error;

// re-export items to simplify their paths when used externally
#[allow(unused_imports)]
pub use jwt::*;

// makes Error and Result accessible via `crate::auth::Error` and `crate::auth::Result`
#[allow(unused_imports)]
pub use error::{Error, Result}; 
