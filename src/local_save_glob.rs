use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct LocalSaveOptions {
    remoteBackupKey: String,
    saveFolderPath: String,
    saveIgnoreGlob: Vec<String>,
}

pub fn glob_save() {}
