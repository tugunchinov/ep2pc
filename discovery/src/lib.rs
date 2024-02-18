#[cfg(test)]
mod tests;

use std::collections::HashSet;
use std::net::SocketAddrV4;

// TODO: add persistent storage for peers
pub struct DiscoveryService {
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

impl Default for DiscoveryService {
    fn default() -> Self {
        Self::new()
    }
}
