use chrono::{DateTime, Utc};
use api::Api;
use crate::errors;
use crate::errors::IoError;
use crate::file::executable::ExitCode;
use crate::path::Path;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum FileType {
    Symlink,
    Executable,
    File,
    Directory
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
pub struct Permissions {
    p: u8
}

impl Permissions {
    pub fn new(p: u8) -> Self {
        Self { p }
    }

    pub fn get(&self) -> u8 {
        self.p
    }

    pub fn is_executable(&self) -> bool {
        self.p & 0b100 != 0
    }

    pub fn is_writable(&self) -> bool {
        self.p & 0b010 != 0
    }

    pub fn is_readable(&self) -> bool {
        self.p & 0b001 != 0
    }

    pub fn set_executable(&mut self, e: bool) {
        if e { self.p |= 0b100; }
        else { self.p &= 0b011; }
    }

    pub fn set_writable(&mut self, w: bool) {
        if w { self.p |= 0b010; }
        else { self.p &= 0b101; }
    }

    pub fn set_readable(&mut self, r: bool) {
        if r { self.p |= 0b001; }
        else { self.p &= 0b110; }
    }
}

/// Contains the metadata of a file
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Metadata {
    file_type: FileType,
    permissions: Permissions,
    created: DateTime<Utc>,
    edited: Option<DateTime<Utc>>,
}

impl Metadata {
    pub fn new(file_type: FileType, permissions: Permissions, created: DateTime<Utc>) -> Self {
        Self { file_type, permissions, created, edited: None }
    }

    pub fn get_file_type(&self) -> FileType {
        self.file_type
    }

    pub fn get_permissions(&self) -> Permissions {
        self.permissions
    }

    pub fn get_created(&self) -> DateTime<Utc> {
        self.created
    }

    pub fn get_edited(&self) -> Option<DateTime<Utc>> {
        self.edited
    }

    pub fn set_permissions(&mut self, permissions: Permissions) {
        self.permissions = permissions;
    }

    pub fn set_edited(&mut self, edited: DateTime<Utc>) {
        self.edited = Some(edited);
    }

    pub fn set_file_type(&mut self, file_type: FileType) {
        self.file_type = file_type;
    }

    pub fn set_created(&mut self, created: DateTime<Utc>) {
        self.created = created;
    }
}

pub struct File {
    metadata: Metadata,
    content: Vec<u8>,
    executable: Option<Box<dyn executable::ExecutableFunction>>
}

impl File {
    pub fn new(metadata: Metadata, content: Vec<u8>, executable: Option<Box<dyn executable::ExecutableFunction>>) -> Self {
        Self { metadata, content, executable }
    }

    pub fn get_metadata(&self) -> Metadata {
        self.metadata
    }

    pub fn get_content(&self) -> &[u8] {
        &self.content
    }
    
    pub fn get_content_utf8(&self, path: Path) -> Result<&str, IoError> {
        match std::str::from_utf8(&self.content) {
            Ok(s) => Ok(s),
            Err(_) => Err(IoError::new_not_utf8(path))
        }
    }

    pub fn get_executable(&self) -> Option<&dyn executable::ExecutableFunction> {
        self.executable.as_ref().map(|e| e.as_ref())
    }

    pub fn set_metadata(&mut self, metadata: Metadata) {
        self.metadata = metadata;
    }

    pub fn set_content(&mut self, content: Vec<u8>) {
        self.content = content;
    }

    pub fn set_executable(&mut self, executable: Option<Box<dyn executable::ExecutableFunction>>) {
        self.executable = executable;
    }

    pub fn execute(&self, api: &mut Api, args: Vec<String>, path: Path) -> Result<ExitCode, IoError> {
        match &self.executable {
            Some(executable) => Ok(executable.execute(api, args)),
            None => Err(IoError::new_not_an_executable(path))
        }
    }
}

pub mod executable {
    use api::Api;

    pub type ExitCode = i32;

    pub trait ExecutableFunction: Send + Sync {
        fn execute(&self, api: &mut Api, args: Vec<String>) -> ExitCode;
    }

    impl ExecutableFunction for fn(&mut Api, Vec<String>) -> ExitCode {
        fn execute(&self, api: &mut Api, args: Vec<String>) -> ExitCode {
            self(api, args)
        }
    }

    impl ExecutableFunction for fn(&mut Api, Vec<String>) -> u8 {
        fn execute(&self, api: &mut Api, args: Vec<String>) -> ExitCode {
            self(api, args).into()
        }
    }
}