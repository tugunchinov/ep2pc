#[cfg(test)]
mod tests;

use std::collections::HashSet;
use std::net::SocketAddrV4;

// TODO: add persistent storage for peers
pub struct DiscoveryService {
    peers: HashSet<SocketAddrV4>,
}

impl DiscoveryService {
    pub fn new(cfg: &settings::discovery::Config) -> Self {
        let peers = if let Some(sync_with) = cfg.sync_with {
            Self::sync_peers(&[sync_with])
        } else {
            HashSet::default()
        };

        Self { peers }
    }

    fn sync_peers(sync_with: &[SocketAddrV4]) -> HashSet<SocketAddrV4> {
        // TODO: better

        let sync_result = HashSet::new();

        for peer in sync_with {
            let request = network::messages::discovery::requests::SyncPeersRequest { garbage: 42 };
            let msg_type = network::messages::discovery::requests::RequestType::SyncPeers;

            network::send_message(*peer, msg_type.into(), request);
        }

        sync_result
    }

    pub fn dispatch_message(
        &self,
        msg_type: network::messages::discovery::DiscoveryMessageType,
        msg_buf: &[u8],
    ) {
        use network::messages::ProtoMessage;

        match msg_type {
            network::messages::discovery::DiscoveryMessageType::RequestType(r) => match r {
                network::messages::discovery::requests::RequestType::SyncPeers => {
                    let request =
                        network::messages::discovery::requests::SyncPeersRequest::decode(msg_buf)
                            .expect("failed decoding request");

                    log::info!("Received SyncPeers request: {request:#?}");
                }
            },
            network::messages::discovery::DiscoveryMessageType::ResponseType(_) => todo!(),
        }
    }

    pub fn add_peer(&mut self, peer: &SocketAddrV4) {
        self.peers.insert(*peer);
    }

    fn get_known_peers(&self, _limit: Option<u64>) -> &HashSet<SocketAddrV4> {
        &self.peers
    }

    pub fn get_random_peers(&self, cnt: u64) -> &HashSet<SocketAddrV4> {
        self.get_known_peers(Some(cnt))
    }

    pub fn discover_new_peers(&mut self) {
        unimplemented!()
    }
}
