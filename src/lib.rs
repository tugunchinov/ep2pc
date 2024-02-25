// TODO: result

use discovery::DiscoveryService;
use peer::Peer;

#[cfg(test)]
mod tests;

pub fn run() {
    env_logger::init();

    let config = settings::Config::new();

    let discovery = DiscoveryService::new(&config.discovery);

    let peer = Peer::new(&config.peer, discovery);

    peer.run();
}
