use std::error::Error;
use std::fmt::Display;
use crate::path::Path;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IoError {
    message: String,
    code: u8,
    file: Option<Path>,
}

impl IoError {
    pub fn new(message: String, code: u8, file: Option<Path>) -> Self {
        Self { message, code, file }
    }

    pub fn get_code(&self) -> u8 {
        self.code
    }

    pub fn get_message(&self) -> &str {
        &self.message
    }


    pub fn get_file(&self) -> Option<&Path> {
        self.file.as_ref()
    }

    pub fn new_cannot_write(file: Path) -> Self {
        Self::new("Cannot write to file".to_string(), 101, Some(file))
    }

    pub fn new_cannot_read(file: Path) -> Self {
        Self::new("Cannot read file".to_string(), 102, Some(file))
    }

    pub fn new_not_found(file: Path) -> Self {
        Self::new("File not found".to_string(), 103, Some(file))
    }

    pub fn new_not_utf8(file: Path) -> Self {
        Self::new("File is not UTF-8".to_string(), 104, Some(file))
    }

    pub fn new_not_an_executable(file: Path) -> Self {
        Self::new("File is not an executable".to_string(), 105, Some(file))
    }
}

impl Display for IoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IO error ({}): {}", self.code, self.message)
    }
}

impl Error for IoError {}