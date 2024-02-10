use std::collections::HashSet;
use std::net::Ipv4Addr;

struct DiscoveryService {
    peers: HashSet<Ipv4Addr>,
}

impl DiscoveryService {
    pub fn new() -> Self {
        Self {
            peers: HashSet::default(),
        }
    }

    pub fn add_peer(&mut self, peer: &Ipv4Addr) {
        self.peers.insert(*peer);
    }

    pub fn get_known_peers(&self) -> &HashSet<Ipv4Addr> {
        &self.peers
    }

    pub fn discover_new_peers(&mut self) {
        unimplemented!()
    }
}
