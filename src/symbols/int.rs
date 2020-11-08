#[derive(Debug)]
pub struct SInt(u32);

impl super::Symbol for SInt {
    fn new(from: &str) -> Option<Self> {
        if let Ok(n) = from.parse::<u32>() {
            Some(SInt(n))
        } else {
            None
        }
    }
}
