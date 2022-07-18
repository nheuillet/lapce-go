use std::env;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use lapce_plugin::{register_plugin, start_lsp, LapcePlugin};

#[derive(Default)]
struct State {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInfo {
    arch: String,
    os: String,
    configuration: Configuration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Configuration {
    language_id: String,
    system_lsp: bool,
    options: Option<Value>,
}

register_plugin!(State);

impl LapcePlugin for State {
    fn initialize(&mut self, info: serde_json::Value) {
        eprintln!("[lapce-go] starting plugin");
        let info = serde_json::from_value::<PluginInfo>(info).unwrap();

        let exec_path = if !info.configuration.system_lsp {
            let go_bin_path = match env::var("GOBIN") {
                Ok(var) => var,
                Err(error) => match error {
                    env::VarError::NotPresent => match env::var("GOPATH") {
                        Ok(var) => format!("{var}/bin"),
                        Err(error) => {
                            panic!("Couldn't get GOPATH: {error}")
                        }
                    },
                    env::VarError::NotUnicode(val) => {
                        let val = val.to_string_lossy();
                        panic!("GOBIN is not in unicode format: '{val}'")
                    }
                },
            };

            format!("{}/gopls", go_bin_path.trim_matches('"'))
        } else {
            String::from("gopls")
        };

        eprintln!("[lapce-go] exec path: {}", exec_path);

        start_lsp(
            &exec_path,
            info.configuration.language_id.as_str(),
            info.configuration.options,
            info.configuration.system_lsp,
        )
    }
}
