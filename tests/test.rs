mod tests_common;
use local_cloud_game_sync::commands::CheckSyncResult;

use crate::tests_common::test_sync_client::TestSyncClient;

#[test]
pub fn initial_upload_test() {
    let client = TestSyncClient::builder()
        .with_empty_remote()
        .with_sync_key("__testKey")
        .with_local_test_folder1()
        .build();

    // Act
    let sync_result = client.check_sync().unwrap();
    assert_eq!(sync_result, CheckSyncResult::RemoteEmpty);

    client.push().expect("Failed to push");

    // Assert
    client.assert_snapshot_count(1);
    client.assert_local_data_matches_remote_data();
    client.assert_is_snapshot_restorable_and_matches_local_data();
    client.assert_local_head_and_remote_head_matches_local_data();
}
