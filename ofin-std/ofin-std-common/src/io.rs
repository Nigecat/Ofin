///! Common io functionality
use ofin_types::OfinString;

/// Print to the standard output with a newline
///
/// # Arguments
///
/// * `text` - The text to print
pub fn print(text: OfinString) {
    println!("{}", text.to_string());
}
