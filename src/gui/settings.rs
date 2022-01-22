use super::support;

use imgui::*;
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
        std::fs::write(pwd.join("Plugins").join("memflow.toml"), &configstr).map_err(|_| {
            Error(ErrorOrigin::Other, ErrorKind::NotFound).log_error("unable to write config file")
        })?;
        Ok(())
    }

    /// Retrieves the current config
    pub fn config(&self) -> Config {
        self.config.clone()
    }

    /// Displays the configuration UI to the user and updates the config
    /// This function blocks until the user clicks the "Ok" button.
    pub fn configure(&mut self) {
        let inventory = Inventory::scan();
        let connectors: Vec<ImString> = inventory
            .available_connectors()
            .iter()
            .map(|c| ImString::from(c.to_owned()))
            .collect::<Vec<_>>();

        let mut connector_idx = connectors
            .iter()
            .enumerate()
            .find(|(_, c)| c.to_str() == self.config.connector)
            .map(|(i, _)| i as i32)
            .unwrap_or_default();
        let mut connector_args = ImString::from(self.config.args.clone());
        let mut log_level_idx = match self.config.log_level.to_lowercase().as_ref() {
            "off" => 0,
            "error" => 1,
            "warn" => 2,
            "info" => 3,
            "debug" => 4,
            "trace" => 5,
            _ => 0,
        };
        let mut parse_sections = self.config.parse_sections;

        {
            support::show_window("memflow", 400.0, 290.0, |run, ui| {
                let connectors_ref: Vec<&ImStr> =
                    connectors.iter().map(|c| c.as_ref()).collect::<Vec<_>>();

                Window::new(im_str!("memflow"))
                    .position([10.0, 10.0], Condition::Always)
                    .size([375.0, 1000.0], Condition::Always)
                    .title_bar(false)
                    .resizable(false)
                    .movable(false)
                    .scroll_bar(false)
                    .save_settings(false)
                    .focus_on_appearing(false)
                    .movable(false)
                    .build(ui, || {
                        ui.text(im_str!("Inventory"));
                        ui.separator();

                        ui.list_box(
                            im_str!("Connector"),
                            &mut connector_idx,
                            &connectors_ref[..],
                            4,
                        );

                        ui.input_text(im_str!("Args"), &mut connector_args).build();

                        ui.dummy([0.0, 16.0]);

                        ui.text(im_str!("Options"));
                        ui.separator();

                        ComboBox::new(im_str!("Log Level")).build_simple_string(
                            ui,
                            &mut log_level_idx,
                            &[
                                im_str!("Off"),
                                im_str!("Error"),
                                im_str!("Warn"),
                                im_str!("Info"),
                                im_str!("Debug"),
                                im_str!("Trace"),
                            ],
                        );

                        ui.checkbox(im_str!("Parse Sections"), &mut parse_sections);

                        // TODO: configure caching

                        ui.dummy([0.0, 16.0]);

                        if ui.button(im_str!("Load"), [64.0, 26.0]) {
                            // update config
                            self.config.connector = connectors
                                .get(connector_idx as usize)
                                .map(|c| c.to_string())
                                .unwrap_or_default();
                            self.config.args = connector_args.to_str().to_owned();
                            self.config.log_level = match log_level_idx {
                                0 => "off",
                                1 => "error",
                                2 => "warn",
                                3 => "info",
                                4 => "debug",
                                5 => "trace",
                                _ => "off",
                            }
                            .to_string();
                            self.config.parse_sections = parse_sections;

                            // close window
                            *run = false;
                        }

                        ui.same_line(64.0 + 16.0);

                        if ui.button(im_str!("Cancel"), [64.0, 26.0]) {
                            *run = false;
                        }
                    });
            });
        }
    }
}
