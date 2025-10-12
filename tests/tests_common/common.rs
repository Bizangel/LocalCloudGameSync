// hardcoded docker values
pub const TEST_SSH_HOST: &str = "testuser@localhost";
pub const TEST_SSH_PORT: u32 = 2222;

// Source folders for testing
pub const TEST_SYNC_FOLDER_DATA_PATH_1: &str = "./docker_test/test_folders/test1";

pub const REMOTE_CONTAINER_INTERNAL_ROOT_FOLDER_PATH: &str = "/home/testuser/testsaves";

pub const TESTING_RESOURCES_ROOT: &str = "./temp_tests";
pub const REMOTE_TEST_ROOT_PATH: &str = "./temp_tests/temp_remote"; // not deriving name from above as this is hardcoded in the container setup - if that's ever changed this needs to match the folder volume that exposes the container
