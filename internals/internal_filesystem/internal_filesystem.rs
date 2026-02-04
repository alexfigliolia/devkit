use normalize_path::NormalizePath;
use std::path::{Path, PathBuf};

pub struct InternalFileSystem {
    root: String,
}

impl InternalFileSystem {
    pub fn new(root: &str) -> InternalFileSystem {
        InternalFileSystem {
            root: root.to_string(),
        }
    }

    pub fn absolute(&self, segment: &str) -> PathBuf {
        let path = Path::new(&self.root);
        path.join(segment).normalize()
    }

    pub fn resolve_command(&self, file_name: &str) -> String {
        self.path_buf_to_str(self.commands_directory().join(file_name))
    }

    pub fn resolve_template(&self, file_name: &str) -> String {
        self.path_buf_to_str(self.templates_directory().join(file_name))
    }

    fn commands_directory(&self) -> PathBuf {
        self.absolute(format!("{}/commands", self.package_directory()).as_str())
    }

    fn templates_directory(&self) -> PathBuf {
        self.absolute(format!("{}/templates", self.package_directory()).as_str())
    }

    fn package_directory(&self) -> String {
        format!("./node_modules/{}/externals", self.package_name())
    }

    fn package_name(&self) -> String {
        "@repokit/core".to_string()
    }

    fn path_buf_to_str(&self, buffer: PathBuf) -> String {
        buffer
            .into_os_string()
            .into_string()
            .expect("Cannot construct path")
    }
}
