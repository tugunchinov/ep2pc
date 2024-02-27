#[derive(Debug)]
pub enum DiscoveryMessageType {
    RequestType(requests::RequestType),
    ResponseType(responses::ResponseType),
}

pub mod requests {
    #[derive(Debug)]
    pub enum RequestType {
        SyncPeers = 0,
    }

    include!(concat!(env!("OUT_DIR"), "/models.discovery.requests.rs"));
}

pub mod responses {
    #[derive(Debug)]
    pub enum ResponseType {
        SyncPeers = 100,
    }

    include!(concat!(env!("OUT_DIR"), "/models.discovery.responses.rs"));
}
