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

fn strip_quotes(var: String) -> String {
    match var.strip_prefix("\"") {
        Some(stripped_var) => match stripped_var.strip_suffix("\"") {
            Some(stripped_var) => stripped_var.to_owned(),
            None => var,
        },
        None => var,
    }
}

impl LapcePlugin for State {
    fn initialize(&mut self, info: serde_json::Value) {
        eprintln!("[lapce-go] starting plugin");
        let info = serde_json::from_value::<PluginInfo>(info).unwrap();

        let exec_path = if !info.configuration.system_lsp {
            let go_bin_path = match env::var("GOBIN").map(strip_quotes) {
                Ok(var) => var,
                Err(error) => match error {
                    env::VarError::NotPresent => String::from(""),
                    env::VarError::NotUnicode(val) => {
                        let val = val.to_string_lossy();
                        panic!("GOBIN is not in unicode format: '{val}'")
                    }
                },
            };

            let go_bin_path = if go_bin_path.is_empty() {
                match env::var("GOPATH").map(strip_quotes) {
                    Ok(var) => format!("{var}/bin"),
                    Err(error) => {
                        panic!("Couldn't get GOPATH: {error}")
                    }
                }
            } else {
                go_bin_path
            };

            format!("{}/gopls", go_bin_path)
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
