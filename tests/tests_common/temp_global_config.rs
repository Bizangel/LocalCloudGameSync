use std::fs;
use std::path::Path;
use std::path::PathBuf;

use super::common::TEST_GLOBAL_CONFIG_PATH;
use super::common::get_test_global_config;

pub struct TempGlobalConfig {
    pub override_path: PathBuf,
}

impl TempGlobalConfig {
    pub fn get_global_config() -> TempGlobalConfig {
        let path = Path::new(TEST_GLOBAL_CONFIG_PATH);
        fs::write(
            path,
            serde_json::to_string_pretty(&get_test_global_config()).unwrap(),
        )
        .expect("Couldn't write test global config");
        return TempGlobalConfig {
            override_path: path.to_path_buf(),
        };
    }
}

impl Drop for TempGlobalConfig {
    fn drop(&mut self) {
        fs::remove_file(&self.override_path).expect("Unable to clean temp test file config");
    }
}
