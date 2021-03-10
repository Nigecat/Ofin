use super::{OfinError, Program};
use std::path::PathBuf;

impl Program {
    /// Parse an ofin program from a file
    pub fn parse_from_file(file: PathBuf) -> Result<Self, OfinError> {
        unimplemented!();
    }

    /// Parse an ofin program from a string
    ///
    /// NOTE: `import` statements will cause a runtime error (`using` statements are fine)
    pub fn parse_from_str(source: String) -> Result<Self, OfinError> {
        unimplemented!();
    }
}
