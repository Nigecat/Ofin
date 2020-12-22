//! The ofin standard library.
//!
//! ## Reading the documentation
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
//! ## Function signatures
//!
//! Ofin has two primitive types;
//!  - string (utf-8)
//!  - int (an unsigned 64 bit or 32 bit integer depending on the platform)
//!
//! TODO

pub use ofin_std_common::*;

#[doc(hidden)]
pub use ofin_types::*;

#[doc(hidden)]
pub mod prelude;

#[cfg(windows)]
pub use ofin_std_windows::*;

#[cfg(unix)]
pub use ofin_std_unix::*;
