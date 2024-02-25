pub mod discovery;
pub mod peer;

pub use prost::Message;

#[derive(Debug)]
pub enum MessageType {
    Discovery(discovery::Discovery),
}

// TODO: TryFrom
impl From<u16> for MessageType {
    fn from(value: u16) -> Self {
        match value {
            0..=999 => MessageType::Discovery(match value {
                0..=99 => discovery::Discovery::Request(match value {
                    0 => discovery::requests::Request::SyncPeers,
                    _ => panic!("unknown discovery request type"),
                }),
                100..=199 => discovery::Discovery::Response(match value {
                    100 => discovery::responses::Response::SyncPeers,
                    _ => panic!("unknown discovery response type"),
                }),
                _ => panic!("unknown discovery message type"),
            }),
            _ => panic!("unknown message type"),
        }
    }
}
