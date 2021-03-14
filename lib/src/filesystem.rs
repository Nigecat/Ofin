//! Filesystem manipulation operations.

module::export! {
    use std::{fs, io, fmt};
    use std::io::Write;
    use std::path::Path;
    
    /// Read the contents of a file.
    pub fn read_file<P: AsRef<Path>>(file: P) -> Result<String, io::Error> {
        fs::read_to_string(file)
    }
    
    /// Write the contents of a file.
    /// 
    /// This will overwrite any existing content or create the file if it does not already exist.
    pub fn write_file<P: AsRef<Path>, F: fmt::Display>(file: P, content: F) -> Result<(), io::Error> {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .open(file)?;
        write!(file, "{}", content)?;
        Ok(())
    }
    
    /// Append to the contents of a file.
    /// 
    /// This will create the file if it does not already exist.
    pub fn append_file<P: AsRef<Path>>(file: P, content: String) -> Result<(), io::Error> {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .open(file)?;
        writeln!(file, "{}", content)?;
        Ok(())
    }
    
    /// Append to the contents of a file with a newline character.
    /// 
    /// This will create the file if it does not already exist.
    /// The supplied text will be appended with a line feed character (`\n`/`U+000A`).
    pub fn appendln_file<P: AsRef<Path>, F: fmt::Display>(file: P, content: F) -> Result<(), io::Error> {
        let mut file = fs::OpenOptions::new()
            .append(true)
            .open(file)?;
        writeln!(file, "{}", content)?;
        Ok(())
    }
}
