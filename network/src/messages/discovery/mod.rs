use crate::messages::MessageType;

#[derive(Debug)]
pub enum DiscoveryMessageType {
    RequestType(requests::RequestType),
    ResponseType(responses::ResponseType),
}

impl From<DiscoveryMessageType> for u16 {
    fn from(value: DiscoveryMessageType) -> Self {
        match value {
            DiscoveryMessageType::RequestType(t) => t as u16,
            DiscoveryMessageType::ResponseType(t) => t as u16,
        }
    }
}

impl<T: Into<DiscoveryMessageType>> From<T> for MessageType {
    fn from(value: T) -> Self {
        let discovery_value: DiscoveryMessageType = value.into();
        MessageType::DiscoveryMessageType(discovery_value)
    }
}

pub mod requests {
    use crate::messages::discovery::DiscoveryMessageType;

    #[derive(Debug)]
    pub enum RequestType {
        SyncPeers = 0,
    }

    impl From<RequestType> for DiscoveryMessageType {
        fn from(value: RequestType) -> Self {
            DiscoveryMessageType::RequestType(value)
        }
    }

    include!(concat!(env!("OUT_DIR"), "/models.discovery.requests.rs"));
}

pub mod responses {
    use crate::messages::discovery::DiscoveryMessageType;

    #[derive(Debug)]
    pub enum ResponseType {
        SyncPeers = 100,
    }

    impl From<ResponseType> for DiscoveryMessageType {
        fn from(value: ResponseType) -> Self {
            DiscoveryMessageType::ResponseType(value)
        }
    }

    include!(concat!(env!("OUT_DIR"), "/models.discovery.responses.rs"));
}
