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

pub use ofin_std_common::*;
pub use ofin_types::*;

#[doc(hidden)]
pub mod prelude;

#[cfg(windows)]
pub use ofin_std_windows::*;

#[cfg(unix)]
pub use ofin_std_unix::*;
