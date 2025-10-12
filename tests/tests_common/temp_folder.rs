use std::fs;
use std::path::PathBuf;

pub struct TestTempFolder {
    pub path: PathBuf,
}

impl TestTempFolder {
    pub fn from_path(path: PathBuf) -> TestTempFolder {
        TestTempFolder { path: path }
    }
}

impl Drop for TestTempFolder {
    fn drop(&mut self) {
        if self.path.exists() && self.path.is_dir() {
            fs::remove_dir_all(&self.path).expect("Unable to clean temp test file config");
        }
    }
}
