use crate::{discovery, peer};

#[derive(serde::Deserialize, Clone, Debug)]
pub struct Config {
    pub peer: peer::Config,
    pub discovery: discovery::Config,
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

        cfg_builder = cfg_builder.add_source(
            config::Environment::default()
                .try_parsing(true)
                .separator("__"),
        );

        cfg_builder
            .build()
            .unwrap()
            .try_deserialize::<Config>()
            .expect("error parsing config")
    }
}
