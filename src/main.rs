use std::env;

use serde::{Deserialize, Serialize};
use serde_json::{Value};

use lapce_plugin::{register_plugin, start_lsp, LapcePlugin};

#[derive(Default)]
struct State {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInfo {
    arch: String,
    os: String,
    configuration: Configuration
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Configuration {
    language_id: String,
    options: Option<Value>,
}

register_plugin!(State);

impl LapcePlugin for State {
    fn initialize(&mut self, info: serde_json::Value) {
        eprintln!("Starting lapce-go plugin!");
        let info = serde_json::from_value::<PluginInfo>(info).unwrap();
        let go_bin_path = match env::var("GOBIN") {
            Ok(var) => var,
            Err(error) => match error {
                Err(env::VarError::NotPresent) => {
                    match env::var("GOPATH") {
                        Ok(var) => format!("{var}/bin"),
                        Err(error) => {
                            panic!("Couldn't get GOPATH: {error}")
                        }
                    }
                },
                Err(env::VarError::NotUnicode(val)) => {
                    let val = val.to_string_lossy();
                    panic!("GOBIN is not in unicode format: '{val}'")
                }
            }
        };

        let file_name = format!("{}/gopls", go_bin_path.strip_prefix("\"").unwrap().strip_suffix("\"").unwrap());

        start_lsp(&file_name, info.configuration.language_id.as_str(), info.configuration.options)
    }

}
