pub mod discovery;
pub mod peer;

pub use prost::Message;

#[derive(Debug)]
pub enum MessageType {
    DiscoveryMessageType(discovery::DiscoveryMessageType),
}

// TODO: TryFrom
impl From<u16> for MessageType {
    fn from(value: u16) -> Self {
        match value {
            0..=999 => MessageType::DiscoveryMessageType(match value {
                0..=99 => discovery::DiscoveryMessageType::RequestType(match value {
                    0 => discovery::requests::RequestType::SyncPeers,
                    _ => panic!("unknown discovery request type"),
                }),
                100..=199 => discovery::DiscoveryMessageType::ResponseType(match value {
                    100 => discovery::responses::ResponseType::SyncPeers,
                    _ => panic!("unknown discovery response type"),
                }),
                _ => panic!("unknown discovery message type"),
            }),
            _ => panic!("unknown message type"),
        }
    }
}
