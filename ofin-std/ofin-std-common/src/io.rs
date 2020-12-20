///! Common io functionality

/// Print to the standard output with a newline
///
/// # Arguments
///
/// * `text` - The text to print
pub fn print<Printable: std::fmt::Display>(text: Printable) {
    println!("{}", text);
}
