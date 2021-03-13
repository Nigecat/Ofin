//! Temporal utility functions.

use std::{thread, time};

/// Put the program to sleep for the specified number of milliseconds.
pub fn sleep(ms: u64) {
    thread::sleep(time::Duration::from_millis(ms));
}