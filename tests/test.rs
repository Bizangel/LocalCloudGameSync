mod tests_common;

use local_cloud_game_sync::commands::CheckSyncResult;
use serial_test::serial;

use crate::tests_common::{test_remote::TestRemote, test_sync_client::TestSyncClient};

#[serial]
#[test]
pub fn initial_upload_test() {
    let remote = TestRemote::builder().with_empty_remote().build();
    let client = TestSyncClient::builder()
        .with_client_name("client1")
        .with_sync_key("testKey")
        .with_local_test_folder1()
        .build();

    // Act
    // Remote should be empty - and upload should be successufl.
    let sync_result = client.check_sync().unwrap();
    assert_eq!(sync_result, CheckSyncResult::RemoteEmpty);
    client.push().expect("Failed to push");

    // Assert push was successful.
    client.assert_snapshot_count(&remote, 1);
    client.assert_local_data_matches_remote_data(&remote);
    client.assert_is_last_snapshot_restorable_and_matches_local_data(&remote);
    client.assert_local_head_and_remote_head_matches_local_data(&remote);
}

#[serial]
#[test]
pub fn happy_path_single_device() {
    // setup
    let remote = TestRemote::builder().with_empty_remote().build();
    let client = TestSyncClient::builder()
        .with_client_name("client1")
        .with_sync_key("testKey")
        .with_local_test_folder1()
        .build();
    client.push().expect("Failed setup push"); // both local and remote are up to date

    // Everything is synced - check that program returns up to date from remote. (This allows silent launch)
    let sync_result = client.check_sync().unwrap();
    let pre_play_hash = client.get_local_hash();
    assert_eq!(sync_result, CheckSyncResult::UpToDate);

    // We launch game silently because up to date - save is modified.
    client
        .modify_stored_save()
        .expect("Unable to modify stored save folder");

    // Remote should be able to be fast-forwarded now. (This allows silent upload)
    let sync_result = client.check_sync().unwrap();
    assert_eq!(sync_result, CheckSyncResult::FastForwardRemote);

    client.push().expect("Failed to push post-modify");

    // Assert
    client.assert_snapshot_count(&remote, 3); // Setup snapshot + before write + after write.
    client.assert_local_data_matches_remote_data(&remote);
    client.assert_local_head_and_remote_head_matches_local_data(&remote);
    client.assert_is_last_snapshot_restorable_and_matches_local_data(&remote);
    // Assert pre edit - snapshot was also successful.
    client.assert_second_last_snapshot_is_restorable_and_matches_hash(&remote, &pre_play_hash);
}
