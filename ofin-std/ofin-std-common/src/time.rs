//! Common time-related functionality
use ofin_types::OfinInt;

/// Put the program to sleep for the specified time duration
///
/// # Arguments
///
/// * `ms` - The number of milliseconds to sleep for
pub fn sleep<I: Into<OfinInt>>(ms: I) {
    // Rely on our int type to handle the conversion from isize -> usize
    std::thread::sleep(std::time::Duration::from_millis(ms.into().into()));
}
