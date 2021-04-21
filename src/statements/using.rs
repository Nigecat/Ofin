use super::imp::Statement;

/// Bring a submodule from the standard library into the current namespace
///
/// usage: using <namespace>
#[derive(Debug)]
pub struct Using {
    target: String,
}

impl Statement for Using {
    fn check(s: &str) -> bool {
        unimplemented!();
    }

    fn convert(s: String) -> Self {
        unimplemented!();
    }
}
