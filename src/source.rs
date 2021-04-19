use std::path::PathBuf;

#[derive(Debug)]
pub enum Source {
    String(String),
    File(PathBuf),
}

impl From<String> for Source {
    fn from(s: String) -> Self {
        Source::String(s)
    }
}

impl From<PathBuf> for Source {
    fn from(path: PathBuf) -> Self {
        Source::File(path)
    }
}
