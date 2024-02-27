pub mod discovery;
pub mod peer;

pub use prost::Message as ProtoMessage;

#[derive(Debug)]
pub enum MessageType {
    DiscoveryMessageType(discovery::DiscoveryMessageType),
}

impl From<MessageType> for u16 {
    fn from(value: MessageType) -> Self {
        match value {
            MessageType::DiscoveryMessageType(d) => d.into(),
        }
    }
}

impl TryFrom<u16> for MessageType {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        // TODO: errors

        Ok(match value {
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
        })
    }
}
