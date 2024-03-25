use memflow::prelude::v1::*;
use serde::{Deserialize, Serialize};

// see https://github.com/serde-rs/serde/issues/368
fn default_string_info() -> String {
    "info".to_string()
}
fn default_bool_true() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub connector: String,
    #[serde(default)]
    pub args: String,

    #[serde(default = "default_string_info")]
    pub log_level: String,

    // TODO: expose caching options (lifetimes, etc)
    #[serde(default = "default_bool_true")]
    pub parse_sections: bool,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            connector: String::new(),
            args: String::new(),

            log_level: "info".to_string(),

            parse_sections: false,
        }
    }
}

pub struct Settings {
    config: Config,
}

impl Settings {
    /// Loads the current config from the {PWD}/Plugins/memflow.toml file.
    pub fn new() -> Self {
        // load config file
        let pwd = std::env::current_dir().expect("unable to get pwd");
        let config = if let Ok(configstr) =
            std::fs::read_to_string(pwd.join("Plugins").join("memflow.toml"))
        {
            toml::from_str::<Config>(&configstr).unwrap_or_default()
        } else {
            Config::default()
        };

        Self { config }
    }

    /// Saves the current configuration to the {PWD}/Plugins/memflow.toml file.
    pub fn persist(&self) -> Result<()> {
        let pwd = std::env::current_dir().map_err(|_| {
            Error(ErrorOrigin::Other, ErrorKind::Unknown).log_error("unable to get pwd")
        })?;
        let configstr = toml::to_string_pretty(&self.config).map_err(|_| {
            Error(ErrorOrigin::Other, ErrorKind::Configuration)
                .log_error("unable to serialize config")
        })?;
        std::fs::write(pwd.join("Plugins").join("memflow.toml"), configstr).map_err(|_| {
            Error(ErrorOrigin::Other, ErrorKind::NotFound).log_error("unable to write config file")
        })?;
        Ok(())
    }

    /// Retrieves the current config
    pub fn config(&self) -> Config {
        self.config.clone()
    }
}
