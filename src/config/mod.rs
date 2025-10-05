mod internal;
mod v1;

use std::path::PathBuf;

use serde::Deserialize;

// const MIN_SUPPORTED_VERSION: &str = "1";
// const MAX_SUPPORTED_VERSION: &str = "1";
// const CURRENT_VERSION: &str = "1";

// #[derive(Debug, Deserialize)]
// pub struct ConfigVersion {
//     pub version: String,
// }

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "version")]
pub enum Config {
    #[serde(rename = "1")]
    V1(v1::ConfigV1),
}

impl Config {
    pub fn load(kuro_dir: Option<PathBuf>) -> anyhow::Result<internal::ConfigInternal> {
        let kuro_dir = match kuro_dir {
            Some(val) => val,
            None => {
                let home_dir = dirs::home_dir()
                    .ok_or(anyhow::anyhow!("User home directory not dectected!"))?;
                home_dir.join(".kuro")
            }
        };

        if !kuro_dir.try_exists()? {
            return Err(anyhow::anyhow!(
                "Kuro directory does not exist: {}",
                kuro_dir.display()
            ));
        }

        let config_path = kuro_dir.join("kuro.toml");

        if !config_path.try_exists()? {
            return Err(anyhow::anyhow!(
                "Kuro config file does not exist: {}",
                config_path.display()
            ));
        }

        let config_str = std::fs::read_to_string(&config_path)?;

        let config: Config = toml::from_str(&config_str)?;

        match config {
            Config::V1(c) => c.to_internal(),
        }
    }
}
