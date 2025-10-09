use local_cloud_game_sync::config::GlobalSaveOptionsJson;

pub const REMOTE_CONTAINER_SAVE_FOLDER_PATH: &str = "/home/testuser/testsaves";
pub const TEST_GLOBAL_CONFIG_PATH: &str = "./test_global_config.json";
pub const LOCAL_TEST_CONFIG_SAVE_KEY: &str = "__temp_test_config";

pub fn get_test_global_config() -> GlobalSaveOptionsJson {
    return GlobalSaveOptionsJson {
        ssh_host: "testuser@localhost".to_string(),
        ssh_port: Some(2222),
        remote_save_folder_path: REMOTE_CONTAINER_SAVE_FOLDER_PATH.to_string(),
    };
}

// cleanup reset remote

// pub fn reset_remote() {
//     let saves_path = Path::new("./temp_remote/GameSaves");

//     fs::remove_dir_all().expect("no error");
//     fs::remove_dir_all(Path::new("./temp_remote/GameSaves")).expect("no error");
//     delete_head_files(Path::new("./temp_remote/.cloudmeta")).expect("no error");
// }

// pub fn delete_head_files(dir: &Path) -> io::Result<()> {
//     // Build a glob matcher for "*.HEAD"
//     let mut builder = GlobSetBuilder::new();
//     builder.add(Glob::new("*.HEAD").unwrap());
//     let matcher = builder.build().unwrap();

//     for entry in fs::read_dir(dir)? {
//         let entry = entry?;
//         let path = entry.path();

//         if path.is_file() {
//             if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
//                 if matcher.is_match(file_name) {
//                     fs::remove_file(&path)?;
//                 }
//             }
//         }
//     }

//     Ok(())
// }
