#[cfg(test)]
mod tests;

use std::collections::HashSet;
use std::io::Write;
use std::net::{SocketAddrV4, TcpStream, UdpSocket};

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
        use messages::Message;

        // TODO: better

        let sync_result = HashSet::new();

        for peer in sync_with {
            let mut socket =
                TcpStream::connect(peer).unwrap_or_else(|_| panic!("failed connecting to {peer}"));

            let request = messages::discovery::requests::SyncPeersRequest { garbage: 42 };

            // TODO: better (one allocation)
            let serialized_message = request.encode_to_vec();
            let mut buf = (request.encoded_len() as u64).to_le_bytes().to_vec();
            buf.extend(
                (messages::discovery::requests::RequestType::SyncPeers as u16)
                    .to_le_bytes()
                    .to_vec(),
            );
            buf.extend(serialized_message);

            log::info!("Sending sync request");

            socket
                .write_all(&buf)
                .expect("failed sending message to {peer}");

            //let (addr, peer) = socket.recv_from().expect("failed receiving 8");
        }

        sync_result
    }

    pub fn dispatch_message(
        &self,
        msg_type: messages::discovery::DiscoveryMessageType,
        msg_buf: &[u8],
    ) {
        use messages::Message;

        match msg_type {
            messages::discovery::DiscoveryMessageType::RequestType(r) => match r {
                messages::discovery::requests::RequestType::SyncPeers => {
                    let request = messages::discovery::requests::SyncPeersRequest::decode(msg_buf)
                        .expect("failed decoding request");

                    log::info!("Received SyncPeers request: {request:#?}");
                }
            },
            messages::discovery::DiscoveryMessageType::ResponseType(_) => todo!(),
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
