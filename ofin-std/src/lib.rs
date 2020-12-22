//! The ofin standard library.
//!
//! # Reading the documentation
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
//! # Function signatures
//!
//! Ofin has two primitive types;
//!  - string
//!  - int 
//!
//! ### String
//! 
//! Ofin strings are utf-8 encoded strings denoted by text surrouded by double quotes.
//! In a function signature they are displayed as `Into<String>`.
//! So for example in our [keyboard/send](keyboard/fn.send.html) function, the signature is as follows:
//! ```
//! pub fn send<S>(text: S) 
//! where
//!     S: Into<String>, 
//! ```
//! And so we can see that the `text` argument is a string type.
//! 
//! ### Int
//! 
//! Ofin ints are signed 64 or 32 bit integers depending on the platform.
//! 
//! The function signature is `Into<Int>` and it works identically to strings.
//! An example of a function taking an int can be seen at [time/sleep](time/fn.sleep.html).

pub use ofin_std_common::*;

#[doc(hidden)]
pub use ofin_types::*;

#[doc(hidden)]
pub mod prelude;

#[cfg(windows)]
pub use ofin_std_windows::*;

#[cfg(unix)]
pub use ofin_std_unix::*;
