#[derive(serde::Deserialize, Clone, Debug)]
pub struct Config {
    pub send_period: u64,
    pub port: u16,
}
