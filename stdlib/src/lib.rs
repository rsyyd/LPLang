//! LPLang Standard Library

pub mod core;
pub mod collections;
pub mod io;
pub mod net;
pub mod crypto;
pub mod data;
pub mod time;
pub mod sync;
pub mod test;
pub mod ffi;
pub mod macros;

/// Prelude — auto-imported in every module.
pub mod prelude {
    pub use crate::core::*;
    pub use crate::collections::*;
    pub use crate::io::*;
    pub use crate::sync::*;
    pub use crate::test::*;
}