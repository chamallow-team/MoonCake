use std::collections::HashMap;
use crate::file::File;
use crate::path::Path;

pub struct Node {
    file: File,
    childrens: HashMap<String, Node>,
    files: HashMap<String, File>
}

impl Node {
    pub fn new(file: File) -> Self {
        Self {
            file,
            childrens: HashMap::new(),
            files: HashMap::new()
        }
    }

    pub fn get_self_file(&self) -> &File {
        &self.file
    }

    pub fn get_file_mut(&mut self) -> &mut File {
        &mut self.file
    }

    pub fn get_childrens(&self) -> &HashMap<String, Node> {
        &self.childrens
    }

    pub fn get_childrens_mut(&mut self) -> &mut HashMap<String, Node> {
        &mut self.childrens
    }

    pub fn get_children(&self, name: &str) -> Option<&Node> {
        self.childrens.get(name)
    }

    pub fn get_files(&self) -> &HashMap<String, File> {
        &self.files
    }

    pub fn get_files_mut(&mut self) -> &mut HashMap<String, File> {
        &mut self.files
    }

    pub fn get_file(&self, name: &str) -> Option<&File> {
        self.files.get(name)
    }

    pub fn add_file(&mut self, name: String, file: File) -> Option<File> {
        self.files.insert(name, file)
    }

    pub fn add_child(&mut self, name: String, node: Node) -> Option<Node> {
        self.childrens.insert(name, node)
    }
}

#[derive(Default)]
pub struct Root {
    childrens: HashMap<String, Node>
}

impl Root {
    pub fn get_childrens(&self) -> &HashMap<String, Node> {
        &self.childrens
    }

    pub fn get_childrens_mut(&mut self) -> &mut HashMap<String, Node> {
        &mut self.childrens
    }

    pub fn get_children(&self, name: &str) -> Option<&Node> {
        self.childrens.get(name)
    }

    pub fn get_file(&self, path: Path) -> Option<&File> {
        let mut path_parts = path.get_parts();

        if path_parts.is_empty() {
            return None;
        }

        let mut node = self.get_children(&path_parts.remove(0))?;
        while path_parts.len() > 1 {
            let part = path_parts.remove(0);
            node = node.get_children(&part)?;
        }

        node.get_file(&path_parts[0])
    }

    pub fn add_child(&mut self, name: String, node: Node) -> Option<Node> {
        self.childrens.insert(name, node)
    }
}

#[cfg(test)]
mod test {
    use chrono::Utc;
    use crate::file::{File, FileType, Metadata, Permissions};
    use crate::tree::{Node, Root};

    #[test]
    fn build_tree(){
        let mut root = Root::default();

        let mut node = Node::new(
            File::new(
                Metadata::new(FileType::Directory, Permissions::new(101), Utc::now()),
                Vec::new(),
                None)
        );

        let a_dummy_file = File::new(
            Metadata::new(FileType::File, Permissions::new(101), Utc::now()),
            "Hello world".as_bytes().to_vec(),
            None
        );

        node.add_file("a_dummy_file.txt".to_string(), a_dummy_file);

        root.add_child("bin".to_string(), node);

        // check if the node was added
        assert!(root.get_children("bin").is_some());
        let bin_node = root.get_children("bin").unwrap();

        // check if the file was added
        assert!(bin_node.get_file("a_dummy_file.txt").is_some());
        let a_dummy_file = bin_node.get_file("a_dummy_file.txt").unwrap();

        assert_eq!(a_dummy_file.get_content(), "Hello world".as_bytes());
        assert_eq!(a_dummy_file.get_metadata().get_file_type(), FileType::File);
    }

    #[test]
    fn get_file_in_folder(){
        let mut root = Root::default();

        let mut node = Node::new(
            File::new(
                Metadata::new(FileType::Directory, Permissions::new(101), Utc::now()),
                Vec::new(),
                None)
        );

        let a_dummy_file = File::new(
            Metadata::new(FileType::File, Permissions::new(101), Utc::now()),
            "Hello world".as_bytes().to_vec(),
            None
        );

        node.add_file("a_dummy_file.txt".to_string(), a_dummy_file);

        root.add_child("bin".to_string(), node);

        let path = "/bin/a_dummy_file.txt".into();

        let file = root.get_file(path);
        assert!(file.is_some());
        let file = file.unwrap();

        assert_eq!(file.get_content(), "Hello world".as_bytes());
        assert_eq!(file.get_metadata().get_file_type(), FileType::File);
    }
}