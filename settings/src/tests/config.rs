use crate::Config;
use std::net::SocketAddrV4;
use std::str::FromStr;
use std::sync::Mutex;

// for env variables
static TEST_MUTEX: Mutex<()> = Mutex::new(());

#[test]
fn test_config_toml() {
    let _guard = TEST_MUTEX.lock().expect("failed locking mutex");

    std::fs::copy("../Settings.sample.toml", "Settings.toml")
        .expect("failed copying Settings.toml");

    let config = Config::new();

    assert_eq!(config.peer.send_period, 333);
    assert_eq!(config.peer.port, 65123);

    let expected_address =
        SocketAddrV4::from_str("1.1.1.1:42").expect("failed creating socket address");
    assert_eq!(config.discovery.sync_with, Some(expected_address));

    std::fs::remove_file("Settings.toml").expect("failed removing Settings.toml");
}

#[test]
fn test_config_env() {
    let _guard = TEST_MUTEX.lock().expect("failed locking mutex");

    std::env::set_var("PEER__SEND_PERIOD", "333");
    std::env::set_var("PEER__PORT", "65123");
    std::env::set_var("DISCOVERY__SYNC_WITH", "1.1.1.1:42");

    let config = Config::new();

    assert_eq!(config.peer.send_period, 333);
    assert_eq!(config.peer.port, 65123);

    let expected_address =
        SocketAddrV4::from_str("1.1.1.1:42").expect("failed creating socket address");
    assert_eq!(config.discovery.sync_with, Some(expected_address));

    std::env::remove_var("PEER__SEND_PERIOD");
    std::env::remove_var("PEER__PORT");
    std::env::remove_var("DISCOVERY__SYNC_WITH");
}

#[test]
fn test_overwrite() {
    let _guard = TEST_MUTEX.lock().expect("failed locking mutex");

    std::fs::copy("../Settings.sample.toml", "Settings.toml")
        .expect("failed copying Settings.toml");

    std::env::remove_var("PEER__SEND_PERIOD");
    std::env::set_var("PEER__PORT", "65111");
    std::env::set_var("DISCOVERY__SYNC_WITH", "8.8.8.8:43");

    let config = Config::new();

    assert_eq!(config.peer.send_period, 333);
    assert_eq!(config.peer.port, 65111);

    let expected_address =
        SocketAddrV4::from_str("8.8.8.8:43").expect("failed creating socket address");
    assert_eq!(config.discovery.sync_with, Some(expected_address));

    std::fs::remove_file("Settings.toml").expect("failed removing Settings.toml");

    std::env::remove_var("PEER__SEND_PERIOD");
    std::env::remove_var("PEER__PORT");
    std::env::remove_var("DISCOVERY__SYNC_WITH");
}
