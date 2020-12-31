//! Common io functionality

use ofin_types::OfinString;
use std::io;
use std::io::prelude::*;

/// Print to the standard output with a newline
///
/// # Arguments
///
/// * `text` - The text to print
pub fn print<Printable: std::fmt::Display>(text: Printable) {
    println!("{}", text);
}

/// Print to the standard output without a newline
///
/// # Arguments
///
/// * `text` - The text to print
pub fn printn<Printable: std::fmt::Display>(text: Printable) {
    print!("{}", text);
}

/// Get user input
/// 
/// This functions reads a line from input, converts it to a string (stripping a trailing newline), and returns it.
pub fn input() -> OfinString {
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim_end().to_string().into()
}
