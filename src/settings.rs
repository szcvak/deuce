use config::Config;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Settings {
    pub key: String,
    pub port: u32,
    pub database: String,
    pub max_rank: u32,
}

impl Settings {
    pub fn load(file: &'static str) -> Result<Self, String> {
        let configuration = Config::builder()
            .add_source(config::File::with_name(file))
            .build();

        match configuration {
            Ok(config) => config.try_deserialize().map_err(|e| e.to_string()),
            Err(e) => Err(e.to_string()),
        }
    }
}
