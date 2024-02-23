use discovery::DiscoveryService;
use settings::Config;
use std::collections::HashSet;
use std::net::{SocketAddrV4, UdpSocket};
use std::sync::Arc;
use std::thread;

#[cfg(test)]
mod tests;

// TODO: better
const MESSAGE_SIZE: usize = 1024;

// TODO: move to separate file
pub struct Peer {
    listening_socket: UdpSocket,
    send_period: u64,
    discovery: DiscoveryService,
}

// TODO: anyhow + error handler?
impl Peer {
    pub fn new(cfg: Config) -> Self {
        // TODO: run discover service
        // TODO: better
        let listening_socket = UdpSocket::bind(format!("0.0.0.0:{}", cfg.port))
            .unwrap_or_else(|_| panic!("failed binding to port {}", cfg.port));

        let mut discovery = DiscoveryService::new();

        if let Some(connect_to) = cfg.connect_to {
            discovery.add_peer(&connect_to);
        }

        Self {
            listening_socket,
            send_period: cfg.send_period,
            discovery,
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

        let mut buf = vec![0; MESSAGE_SIZE];

        loop {
            // TODO: better (check if one message is split)
            // TODO: check whether it is a message or an attempt to join the network / sync peers in discovery
            let (_, peer) = self
                .listening_socket
                .recv_from(&mut buf)
                .expect("failed reading message");

            let received_message =
                std::str::from_utf8(&buf).expect("failed parsing incoming message");

            log::info!("Message from {peer}: {received_message}",);

            let peers = self.discovery.get_random_peers(10);
            self.broadcast_message(received_message.as_bytes(), peers);
        }
    }

    fn speak(&self) {
        // TODO: run discovery service
        log::info!("Broadcasting messages...");

        loop {
            let peers = self.discovery.get_random_peers(10);

            let now = std::time::SystemTime::now();
            let me = self
                .listening_socket
                .local_addr()
                .expect("failed getting local address");
            let message = format!("Peer {me}. Time: {now:#?}");

            self.broadcast_message(message.as_bytes(), peers);

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
