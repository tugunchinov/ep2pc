use std::net::SocketAddrV4;

#[derive(serde::Deserialize, Clone, Debug)]
pub struct Config {
    pub sync_with: Option<SocketAddrV4>,
    // TODO: repo
}
