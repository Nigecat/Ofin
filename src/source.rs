use std::convert::TryFrom;
use std::path::PathBuf;
use std::{fs, io};

#[derive(Debug)]
pub enum SourceType {
    String(String),
    File(PathBuf),
}

#[derive(Debug)]
pub struct Source(SourceType);

impl From<String> for Source {
    fn from(s: String) -> Self {
        Source(SourceType::String(s))
    }
}

impl From<PathBuf> for Source {
    fn from(path: PathBuf) -> Self {
        Source(SourceType::File(path))
    }
}

impl TryFrom<Source> for String {
    type Error = io::Error;

    fn try_from(source: Source) -> Result<Self, Self::Error> {
        match source.0 {
            SourceType::String(s) => Ok(s),
            SourceType::File(path) => fs::read_to_string(path),
        }
    }
}
