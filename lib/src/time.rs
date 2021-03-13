//! Temporal utility functions.

use std::{thread, time};

/// Put the program to sleep for the specified amount of time.
pub fn sleep(ms: usize) {
    thread::sleep(time::Duration::from_millis(ms as u64));
}