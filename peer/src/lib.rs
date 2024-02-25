use discovery::DiscoveryService;
use std::collections::HashSet;
use std::net::{SocketAddrV4, UdpSocket};
use std::sync::Arc;
use std::thread;

#[cfg(test)]
mod tests;

// TODO: move to separate file
pub struct Peer {
    discovery: DiscoveryService,
    listening_socket: UdpSocket,
    send_period: u64,
}

// TODO: better error handling
impl Peer {
    pub fn new(cfg: &settings::peer::Config, discovery: DiscoveryService) -> Self {
        // TODO: better
        let listening_socket = UdpSocket::bind(format!("0.0.0.0:{}", cfg.port))
            .unwrap_or_else(|_| panic!("failed binding to port {}", cfg.port));

        Self {
            discovery,
            listening_socket,
            send_period: cfg.send_period,
        }
    }

    // blocking
    pub fn run(self) {
        log::info!("The peer is starting...");

        let arc_self = Arc::new(self);

        let listen_handle = {
            let self_cloned = arc_self.clone();

            thread::spawn(move || self_cloned.listen())
        };

        arc_self.speak();

        listen_handle.join().expect("failed");
    }

    fn listen(&self) {
        log::info!(
            "Listening on {}",
            self.listening_socket
                .local_addr()
                .expect("failed getting local address")
        );

        let mut len_buf = [0u8; 8];
        let mut msg_type_buf = [0u8; 2];

        loop {
            // TODO: better (check if one message is split)
            // TODO: check whether it is a message or an attempt to join the network / sync peers in discovery
            let (_, peer) = self
                .listening_socket
                .recv_from(&mut len_buf)
                .expect("failed reading message length");

            let len = u64::from_le_bytes(len_buf);

            log::info!("received message from peer {peer} with length {len}");

            let (_, peer) = self
                .listening_socket
                .recv_from(&mut msg_type_buf)
                .expect("failed reading message length");

            let msg_type = messages::MessageType::from(u16::from_le_bytes(msg_type_buf));

            let mut message_buf = vec![0u8; len as usize];

            let (bytes_received, peer) = self
                .listening_socket
                .recv_from(&mut message_buf)
                .expect("failed reading message");

            assert_eq!(bytes_received, len as usize);

            log::info!(
                "received message from peer {peer} with length {len}. Message type: {msg_type:#?}"
            );

            // let peers = self.discovery.get_random_peers(10);
            // self.broadcast_message(received_message.as_bytes(), peers);
        }
    }

    fn speak(&self) {
        // TODO: run discovery service
        log::info!("Broadcasting messages...");

        loop {
            // let peers = self.discovery.get_random_peers(10);

            let now = std::time::SystemTime::now();
            let me = self
                .listening_socket
                .local_addr()
                .expect("failed getting local address");
            let message = format!("Peer {me}. Time: {now:#?}");

            //self.broadcast_message(message.as_bytes(), peers);

            thread::sleep(std::time::Duration::from_secs(self.send_period));
        }
    }

    fn broadcast_message(&self, message: &[u8], peers: &HashSet<SocketAddrV4>) {
        let mut handles = Vec::with_capacity(peers.len());

        // TODO: reliable broadcast
        for peer in peers {
            // TODO: green threads
            {
                // satisfying borrow checker
                let peer = *peer;
                let message = message.to_vec();

                handles.push(thread::spawn(move || {
                    let speaking_socket = UdpSocket::bind("0.0.0.0:0")
                        .unwrap_or_else(|_| panic!("failed creating socket"));

                    speaking_socket
                        .connect(peer)
                        .expect("failed connecting to peer {peer}");

                    speaking_socket
                        .send(&message)
                        .expect("failed sending message to {peer}");
                }));
            }
        }

        // TODO: futures::join
        for handle in handles {
            handle.join().expect("failed joining handle");
        }
    }
}
