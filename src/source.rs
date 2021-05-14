use std::convert::TryFrom;
use std::path::PathBuf;
use std::{fs, io};

#[derive(Debug, Clone)]
pub enum Source {
    Static(&'static str),
    String(String),
    File(PathBuf),
}

impl From<&'static str> for Source {
    fn from(source: &'static str) -> Self {
        Source::Static(source)
    }
}

impl From<String> for Source {
    fn from(source: String) -> Self {
        Source::String(source)
    }
}

impl From<PathBuf> for Source {
    fn from(source: PathBuf) -> Self {
        Source::File(source)
    }
}

impl TryFrom<Source> for String {
    type Error = io::Error;

    fn try_from(source: Source) -> Result<Self, Self::Error> {
        match source {
            Source::Static(s) => Ok(s.to_string()),
            Source::String(s) => Ok(s),
            Source::File(p) => fs::read_to_string(p),
        }
    }
}
