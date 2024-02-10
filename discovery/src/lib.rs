use std::collections::HashSet;
use std::net::SocketAddrV4;

struct DiscoveryService {
    peers: HashSet<SocketAddrV4>,
}

impl DiscoveryService {
    pub fn new() -> Self {
        Self {
            peers: HashSet::default(),
        }
    }

    pub fn add_peer(&mut self, peer: &SocketAddrV4) {
        self.peers.insert(*peer);
    }

    pub fn get_known_peers(&self) -> &HashSet<SocketAddrV4> {
        &self.peers
    }

    pub fn discover_new_peers(&mut self) {
        unimplemented!()
    }
}
