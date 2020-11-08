#[derive(Debug)]
pub struct SString(String);

impl super::Symbol for SString {
    fn new(from: &str) -> Option<Self> {
        Some(SString(from.to_string()))
    }
}
