use std::fmt;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct SyntaxError {
    pub file: PathBuf,
    pub line: usize,
    pub ctx: String,
    pub(crate) t: &'static str,
    pub(crate) v: &'static str,
}

impl std::error::Error for SyntaxError {}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "{} {} at {}:{}",
            self.v,
            self.t,
            self.file.display(),
            self.line
        )?;
        write!(f, "{} | ...{}", self.line, self.ctx)
    }
}
