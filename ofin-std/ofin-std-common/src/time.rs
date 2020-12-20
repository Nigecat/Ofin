///! Common time-related functionality
use ofin_types::OfinInt;

/// Put the program to sleep for the specified time duration
///
/// # Arguments
///
/// * `ms` - The number of milliseconds to sleep for
pub fn sleep(ms: OfinInt) {
    std::thread::sleep(std::time::Duration::from_millis(usize::from(ms) as u64));
}
