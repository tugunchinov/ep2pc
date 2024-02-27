use std::io::Write;
use std::net::{SocketAddrV4, TcpStream};

pub mod messages;

pub fn send_message<T: messages::ProtoMessage>(
    peer: SocketAddrV4,
    msg_type: messages::MessageType,
    msg: T,
) {
    let mut socket =
        TcpStream::connect(peer).unwrap_or_else(|_| panic!("failed connecting to {peer}"));

    // TODO: better (one allocation)
    let serialized_message = msg.encode_to_vec();
    let mut buf = (msg.encoded_len() as u64).to_le_bytes().to_vec();
    buf.extend(u16::from(msg_type).to_le_bytes().to_vec());
    buf.extend(serialized_message);

    log::info!("Sending sync request");

    socket
        .write_all(&buf)
        .expect("failed sending message to {peer}");
}
