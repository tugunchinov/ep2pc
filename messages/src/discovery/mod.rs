#[derive(Debug)]
pub enum Discovery {
    Request(requests::Request),
    Response(responses::Response),
}

pub mod requests {
    #[derive(Debug)]
    pub enum Request {
        SyncPeers = 0,
    }

    include!(concat!(env!("OUT_DIR"), "/models.discovery.requests.rs"));
}

pub mod responses {
    #[derive(Debug)]
    pub enum Response {
        SyncPeers = 100,
    }

    include!(concat!(env!("OUT_DIR"), "/models.discovery.responses.rs"));
}
