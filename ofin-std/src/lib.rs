//! The ofin standard library.
//!
//! Ofin standard library imports are as follows:
//! ```
//! import <std/...>;
//! ```
//! Where `...` is the path to the module you wish to import.
//!
//! For example, if you wanted to import the [`time`](time/index.html) submodule, you would do
//! ```
//! import <std/time>;
//! ```
//! This would bring all function inside that module into the current scope.
//!
//! All functions in the [`ofin prelude`](prelude/index.html) are automatically imported for all scripts.

pub use ofin_std_common::*;
pub use ofin_types::*;
pub mod prelude;
