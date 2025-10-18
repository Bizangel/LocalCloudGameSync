use crate::config::config_commons::*;
use globset::GlobSet;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

pub struct ValidatedSyncEntry {
    pub remote_sync_key: String,
    pub save_folder_path: PathBuf,
    pub save_ignore_glob: GlobSet,
    pub display_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SyncEntry {
    pub remote_sync_key: String,
    pub save_folder_path: String,
    pub save_ignore_glob: Vec<String>,
    pub display_name: Option<String>,
}

pub struct ValidatedSyncOptions {
    pub client_name: String,
    pub ssh_host: String,
    pub ssh_port: u32,
    pub remote_sync_root: String,
    pub local_head_folder: PathBuf,
    pub sync_entries: Vec<SyncEntry>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SyncOptionsJson {
    pub client_name: String,
    pub ssh_host: String,
    pub ssh_port: Option<u32>,
    pub remote_sync_root: String,
    pub local_head_folder: Option<String>,
    pub sync_entries: Vec<SyncEntry>,
}

#[path = "./sync_options_validator.rs"]
mod sync_options_validator;
