use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub patreon_campaign_id: String,
    pub patreon_client_id: String,
    pub patreon_client_secret: String,
    pub server_addr: String,
    pub database_uri: String,
    #[serde(default = "returns_true")]
    pub json_log: bool,
}

impl Config {
    pub fn new() -> Result<Config, envy::Error> {
        envy::from_env()
    }
}

fn returns_true() -> bool {
    true
}
