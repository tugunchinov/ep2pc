use discovery::DiscoveryService;
use messages::MessageType;
use std::collections::HashSet;
use std::io::Read;
use std::net::{SocketAddrV4, TcpListener};
use std::sync::Arc;
use std::thread;

#[cfg(test)]
mod tests;

// TODO: move to separate file
pub struct Peer {
    discovery: DiscoveryService,
    listener: TcpListener,
    send_period: u64,
}

// TODO: better error handling
impl Peer {
    pub fn new(cfg: &settings::peer::Config, discovery: DiscoveryService) -> Self {
        // TODO: better
        let listening_socket = TcpListener::bind(format!("0.0.0.0:{}", cfg.port))
            .unwrap_or_else(|_| panic!("failed binding to port {}", cfg.port));

        Self {
            discovery,
            listener: listening_socket,
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
            self.listener
                .local_addr()
                .expect("failed getting local address")
        );

        let mut meta_info_buf = [0u8; 10];

        loop {
            // TODO: better (check if one message is split)
            // TODO: check whether it is a message or an attempt to join the network / sync peers in discovery
            let (mut incoming, peer) = self.listener.accept().expect("failed accepting connection");

            incoming
                .read_exact(&mut meta_info_buf)
                .expect("failed reading message meta");

            let (msg_len_buf, msg_type_buf) = meta_info_buf.split_at(8);

            let msg_len = u64::from_le_bytes(msg_len_buf.try_into().unwrap());
            let msg_type =
                messages::MessageType::from(u16::from_le_bytes(msg_type_buf.try_into().unwrap()));

            log::info!("received message from peer {peer}, type: {msg_type:#?}, length: {msg_len}");

            let mut msg_buf = vec![0u8; msg_len as usize];

            incoming
                .read_exact(&mut msg_buf)
                .expect("failed reading message body");

            match msg_type {
                MessageType::DiscoveryMessageType(t) => {
                    self.discovery.dispatch_message(t, msg_buf.as_slice());
                }
            }

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
                .listener
                .local_addr()
                .expect("failed getting local address");
            let _message = format!("Peer {me}. Time: {now:#?}");

            //self.broadcast_message(message.as_bytes(), peers);

            thread::sleep(std::time::Duration::from_secs(self.send_period));
        }
    }

    fn _broadcast_message(&self, _message: &[u8], _peers: &HashSet<SocketAddrV4>) {
        todo!()
        // let mut handles = Vec::with_capacity(peers.len());
        //
        // // TODO: reliable broadcast
        // for peer in peers {
        //     // TODO: green threads
        //     {
        //         // satisfying borrow checker
        //         let peer = *peer;
        //         let message = message.to_vec();
        //
        //         handles.push(thread::spawn(move || {
        //             let speaking_socket = UdpSocket::bind("0.0.0.0:0")
        //                 .unwrap_or_else(|_| panic!("failed creating socket"));
        //
        //             speaking_socket
        //                 .connect(peer)
        //                 .expect("failed connecting to peer {peer}");
        //
        //             speaking_socket
        //                 .send(&message)
        //                 .expect("failed sending message to {peer}");
        //         }));
        //     }
        // }
        //
        // // TODO: futures::join
        // for handle in handles {
        //     handle.join().expect("failed joining handle");
        // }
    }
}
