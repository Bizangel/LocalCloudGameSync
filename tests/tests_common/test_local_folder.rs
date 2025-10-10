use super::common::{LOCAL_TEST_SAVE_PATH, TEST_SYNC_FOLDER_DATA_PATH_1};
use super::utils::copy_dir_all;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

pub struct TestTempFolder {
    pub path: PathBuf,
}

impl TestTempFolder {
    pub fn with_test_local_folder1() -> TestTempFolder {
        let path = Path::new(LOCAL_TEST_SAVE_PATH).to_path_buf();
        copy_dir_all(TEST_SYNC_FOLDER_DATA_PATH_1, &path)
            .expect("Unable to copy test local folder for testing");
        return TestTempFolder { path };
    }

    pub fn from_path(path: &Path) -> TestTempFolder {
        TestTempFolder {
            path: path.to_path_buf(),
        }
    }
}

impl Drop for TestTempFolder {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.path).expect("Unable to clean temp test file config");
    }
}
