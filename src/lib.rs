// TODO: result

use peer::Peer;

#[cfg(test)]
mod tests;

pub fn run() {
    env_logger::init();

    let config = settings::Config::new();

    let peer = Peer::new(config);

    peer.run();
}
