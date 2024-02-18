use argh::FromArgs;
use serde::Deserialize;
use std::net::SocketAddrV4;

#[derive(FromArgs)]
/// Peer's startup params.
/// All these params might also be set via environment variables or Settings.toml
struct CmdParams {
    #[argh(option, short = 's')]
    /// messages sending period
    send_period: Option<u64>,
    #[argh(option, short = 'p')]
    /// port to listen to
    port: Option<u16>,
    #[argh(option, short = 'c')]
    /// peer to connect to for synchronization
    connect_to: Option<SocketAddrV4>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Config {
    pub send_period: u64,
    pub port: u16,
    pub connect_to: Option<SocketAddrV4>,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    pub fn new() -> Config {
        let mut cfg_builder = config::Config::builder();

        if std::path::Path::new("Settings.toml").exists() {
            cfg_builder = cfg_builder.add_source(config::File::with_name("./Settings.toml"));
        }

        cfg_builder = cfg_builder.add_source(config::Environment::default().try_parsing(true));

        // TODO: better
        let params: CmdParams = argh::from_env();

        cfg_builder = cfg_builder
            .set_override_option("send_period", params.send_period)
            .expect("failed overriding send_period");

        cfg_builder = cfg_builder
            .set_override_option("port", params.port)
            .expect("failed overriding port");

        cfg_builder = cfg_builder
            .set_override_option("connect_to", params.connect_to.map(|c| c.to_string()))
            .expect("failed overriding connect_to");

        cfg_builder
            .build()
            .unwrap()
            .try_deserialize::<Config>()
            .expect("error parsing config")
    }
}
