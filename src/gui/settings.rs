use super::support;

use imgui::*;
use memflow::*;
use memflow_win32::error::Result;
use serde::{Deserialize, Serialize};

// see https://github.com/serde-rs/serde/issues/368
#[allow(unused)]
fn default_as_true() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub connector: String,
    #[serde(default)]
    pub args: String,

    // TODO: expose caching options (lifetimes, etc)
    #[serde(default = "default_as_true")]
    pub parse_sections: bool,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            connector: String::new(),
            args: String::new(),

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
        let pwd = std::env::current_dir().map_err(|_| "unable to get pwd")?;
        let configstr =
            toml::to_string_pretty(&self.config).map_err(|_| "unable to serialize config")?;
        std::fs::write(pwd.join("Plugins").join("memflow.toml"), &configstr)
            .map_err(|_| "unable to write config file")?;
        Ok(())
    }

    /// Retrieves the current config
    pub fn config(&self) -> Config {
        self.config.clone()
    }

    /// Displays the configuration UI to the user and updates the config
    /// This function blocks until the user clicks the "Ok" button.
    pub fn configure(&mut self) {
        let inventory = unsafe { ConnectorInventory::scan() };
        let connectors: Vec<ImString> = inventory
            .available_connectors()
            .iter()
            .map(|c| ImString::from(c.to_owned()))
            .collect::<Vec<_>>();

        let mut connector_idx = connectors
            .iter()
            .enumerate()
            .find(|(_, c)| c.to_str() == self.config.connector)
            .and_then(|(i, _)| Some(i as i32))
            .unwrap_or_default();
        let mut connector_args = ImString::from(self.config.args.clone());
        let mut parse_sections = self.config.parse_sections;

        {
            support::show_window("memflow", 400.0, 265.0, |run, ui| {
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

                        ui.checkbox(im_str!("Parse Sections"), &mut parse_sections);

                        // TODO: configure caching

                        ui.dummy([0.0, 16.0]);

                        if ui.button(im_str!("Load"), [64.0, 26.0]) {
                            // update config
                            self.config.connector = connectors
                                .get(connector_idx as usize)
                                .and_then(|c| Some(c.to_string()))
                                .unwrap_or_default();
                            self.config.args = connector_args.to_str().to_owned();
                            self.config.parse_sections = parse_sections;

                            // close window
                            *run = false;
                        }
                    });
            });
        }
    }
}
