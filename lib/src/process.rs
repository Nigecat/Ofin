//! Process interaction utilities.

use std::process;

/// Terminate the program with the specified exit code.
pub fn exit(code: i32) {
    process::exit(code);
}
