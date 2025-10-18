mod tests_common;

use crate::tests_common::common::TEST_SSH_HOST;
use crate::tests_common::test_sync_client::AssertableCheckSyncResult;
use crate::tests_common::{test_remote::TestRemote, test_sync_client::TestSyncClient};
use serial_test::serial;

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
    // Remote should be empty - and upload should be successful.
    client.check_sync().assert_remote_empty();
    client.push().expect("Failed to push");

    // Assert push was successful.
    client.assert_client_is_remote_author(&remote);
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
    let pre_play_hash = client.get_local_hash();
    client.check_sync().assert_up_to_date();
    // We launch game silently because up to date - save is modified.
    client.modify_stored_save();

    // Remote should be able to be fast-forwarded now. (This allows silent upload)
    client.check_sync().assert_fast_forward_remote();

    client.push().expect("Failed to push post-modify");

    // Assert
    client.assert_client_is_remote_author(&remote);
    client.assert_snapshot_count(&remote, 3); // Setup snapshot + before write + after write.
    client.assert_local_data_matches_remote_data(&remote);
    client.assert_local_head_and_remote_head_matches_local_data(&remote);
    client.assert_is_last_snapshot_restorable_and_matches_local_data(&remote);
    // Assert pre edit - snapshot was also successful.
    client.assert_second_last_snapshot_is_restorable_and_matches_hash(&remote, &pre_play_hash);
}

#[serial]
#[test]
pub fn happy_path_multiple_devices() {
    // setup
    let remote = TestRemote::builder().with_empty_remote().build();
    let client1 = TestSyncClient::builder()
        .with_client_name("client1")
        .with_sync_key("sameKey")
        .with_local_test_folder1()
        .build();

    let client2 = TestSyncClient::builder()
        .with_client_name("client2")
        .with_sync_key("sameKey")
        .with_empty_test_folder()
        .build();

    client1.push().expect("Failed setup push");
    client2.pull().expect("Failed to setup pull");

    // Everyone has same state at this point
    client1.check_sync().assert_up_to_date();
    client2.check_sync().assert_up_to_date();
    client1.assert_local_data_matches_remote_data(&remote);
    client2.assert_local_data_matches_remote_data(&remote);
    client1.assert_client_is_remote_author(&remote); // client 1 is author - client 2 just pulled

    // Plays game
    client1.modify_stored_save();
    // Everyone is up to date so launch game silently. Stops playing.
    client1.check_sync().assert_fast_forward_remote(); // remote can be fast-forwarded.
    client1.push().expect("Failed to push to remote"); // fast forward remote

    // Now remote 2 wants to play
    client2.check_sync().assert_fast_forward_local(); // Should be able to silently fetch
    client2.pull().expect("Failed to pull from remote"); // fast forward from remote.
    client2.modify_stored_save(); // modifies it further
    // Should be able to push silently
    client2.check_sync().assert_fast_forward_remote();
    client2.push().expect("Failed to push");

    // Assert
    client2.assert_snapshot_count(&remote, 5); // Setup snapshot + before write + after write + before client 2 write + after client 2 write.
    client2.assert_local_data_matches_remote_data(&remote);
    client2.assert_local_head_and_remote_head_matches_local_data(&remote);
    client2.assert_is_last_snapshot_restorable_and_matches_local_data(&remote);
    client2.assert_client_is_remote_author(&remote);

    // Assert that old client1 data is restorable and matches
    // Assert pre edit - snapshot was also successful.
    client1.assert_second_last_snapshot_is_restorable_and_matches_hash(
        &remote,
        &client1.get_local_hash(),
    );
}

#[serial]
#[test]
pub fn immutability_test() {
    // Setup
    let _remote = TestRemote::builder().with_empty_remote().build();
    let client = TestSyncClient::builder()
        .with_client_name("client1")
        .with_sync_key("testKey")
        .with_local_test_folder1()
        .build();
    client.push().expect("Failed setup push");
    client.check_sync().assert_up_to_date();

    // Act
    // nothing changes.
    client.check_sync().assert_up_to_date();
}

#[serial]
#[test]
pub fn weird_state_conflict() {
    // Setup
    let remote = TestRemote::builder().with_empty_remote().build();
    let client1 = TestSyncClient::builder()
        .with_client_name("client1")
        .with_sync_key("testKey")
        .with_local_test_folder1()
        .build();
    client1.push().expect("Failed setup push");

    let client2 = TestSyncClient::builder()
        .with_client_name("client2")
        .with_sync_key("testKey")
        .with_empty_test_folder()
        .build();
    client2.pull().expect("Failed setup pull");
    client1.check_sync().assert_up_to_date();
    client2.check_sync().assert_up_to_date();

    // Act
    // client 2 plays remotely - pushes new version to remote
    client2.modify_stored_save();
    client2.push().expect("Unable to push to remote");

    // client 1 - modifies save game manually
    client1.modify_stored_save();

    // client 1 should have a save conflict
    client1.check_sync().assert_conflict();
    client2.assert_client_is_remote_author(&remote); // conflict display for remote author should be client2
}

#[serial]
#[test]
pub fn play_offline_come_back_internet() {
    let remote = TestRemote::builder().with_empty_remote().build();
    let mut client = TestSyncClient::builder()
        .with_client_name("client1")
        .with_sync_key("testKey")
        .with_local_test_folder1()
        .build();

    client.push().expect("Failed setup push");
    client.check_sync().assert_up_to_date();

    // Act
    client.config.ssh_host = "nonexistanthost".to_string();
    client.push().expect_err("Expected failure when pushing");

    // plays anyways offline
    client.modify_stored_save();

    // Back online
    client.config.ssh_host = TEST_SSH_HOST.to_string();
    client.check_sync().assert_fast_forward_remote(); // can fast forward
    client.push().expect("Unable to push post-offline");

    client.assert_client_is_remote_author(&remote);
    client.assert_snapshot_count(&remote, 3); // Setup snapshot + before write + after write.
    client.assert_local_data_matches_remote_data(&remote);
    client.assert_local_head_and_remote_head_matches_local_data(&remote);
    client.assert_is_last_snapshot_restorable_and_matches_local_data(&remote);
}

#[serial]
#[test]
pub fn play_offline_but_accidentally_played_on_other_device_hence_conflict() {
    // Setup
    let remote = TestRemote::builder().with_empty_remote().build();
    let mut client1 = TestSyncClient::builder()
        .with_client_name("client1")
        .with_sync_key("testKey")
        .with_local_test_folder1()
        .build();
    client1.push().expect("Failed setup push");

    let client2 = TestSyncClient::builder()
        .with_client_name("client2")
        .with_sync_key("testKey")
        .with_empty_test_folder()
        .build();

    client2.pull().expect("Failed setup pull");
    client1.check_sync().assert_up_to_date();
    client2.check_sync().assert_up_to_date();

    // Play offline on Client1
    client1.config.ssh_host = "nonexistanthost".to_string();
    client1.push().expect_err("Expected failure when pushing");

    // plays anyways offline
    client1.modify_stored_save();

    // Played online on client2
    client2.modify_stored_save(); // twice to ensure diff from above
    client2.modify_stored_save();

    // should be able to push cleanly - we're unaware of the offline plays of client1
    client2.check_sync().assert_fast_forward_remote();
    client2.push().expect("Unable to push");

    // Back online
    client1.config.ssh_host = TEST_SSH_HOST.to_string();
    client1.check_sync().assert_conflict(); // can fast forward
    client2.assert_client_is_remote_author(&remote); // client 2 is the one who managed to push
}
