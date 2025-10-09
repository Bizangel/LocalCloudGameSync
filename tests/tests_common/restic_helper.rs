use serde::{Deserialize, Serialize};

// #[derive(Serialize, Deserialize, Debug)]
// pub struct ResticSnapshotManifestSummary {
//     pub backup_start: String,
//     pub backup_end: String,
//     pub files_new: u32,
//     pub files_changed: u32,
//     pub files_unmodified: u32,
//     pub dirs_new: u32,
//     pub dirs_changed: u32,
//     pub dirs_unmodified: u32,
//     pub data_blobs: u32,
//     pub data_added: u32,
//     pub data_added_packed: u32,
//     pub total_files_processed: u32,
//     pub total_bytes_processed: u32,
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct ResticSnapshotManifest {
    pub time: String,
    pub tree: String,
    pub paths: Vec<String>,
    pub hostname: String,
    pub username: String,
    pub uid: u32,
    pub gid: u32,
    pub id: String,
    pub short_id: String,
}

#[test]
pub fn restic_snapshot_manifest_sanity_test() {
    let testjson = r#"
        [{
            "time": "2025-10-09T22:03:49.549567703+01:00",
            "tree": "0b6c57628292329ef3aeb52c87c4547f3230d2925c9f2606214c963bc3506811",
            "paths": [
                "/tmp/restic-test"
            ],
            "hostname": "Ark-Thinkpad",
            "username": "arcanzu",
            "uid": 1000,
            "gid": 1000,
            "id": "b65ca0f455320675fa89b75fc061af298d4236835a12f28386ef3b87ef3d8ce9",
            "short_id": "b65ca0f4"
        }]
    "#;

    let parse: Vec<ResticSnapshotManifest> = serde_json::from_str(testjson).unwrap();
    assert_eq!(
        parse[0].id,
        "b65ca0f455320675fa89b75fc061af298d4236835a12f28386ef3b87ef3d8ce9"
    );
}
