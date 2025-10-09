use super::utils::copy_dir_all;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

pub struct TestLocalFolder {
    pub path: PathBuf,
}

impl TestLocalFolder {
    pub fn with_test_folder() -> TestLocalFolder {
        let path = Path::new("./temp_local").to_path_buf();
        copy_dir_all("./docker_test/test_folders/test1", &path)
            .expect("Unable to copy test local folder for testing");
        return TestLocalFolder { path };
    }
}

impl Drop for TestLocalFolder {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.path).expect("Unable to clean temp test file config");
    }
}
