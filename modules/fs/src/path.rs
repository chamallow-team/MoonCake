
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Path(String);

impl Path {
    pub fn new(path: &str) -> Self {
        Self(path.to_string())
    }

    pub fn get_path(&self) -> &str {
        &self.0
    }

    pub fn get_parts(&self) -> Vec<String> {
        self.0.split('/').map(|s| s.to_string()).filter(|s| !s.is_empty()).collect()
    }
}

impl From<String> for Path {
    fn from(path: String) -> Self {
        Self(path)
    }
}

impl From<&str> for Path {
    fn from(path: &str) -> Self {
        Self(path.to_string())
    }
}