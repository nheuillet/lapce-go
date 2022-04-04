// use std::{
//     path::PathBuf,
//     process::Command,
//     env,
// };

use lapce_plugin::{register_plugin, start_lsp, LapcePlugin};
use serde::{Deserialize, Serialize};
use serde_json::{Value};

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
        eprintln!("Starting lapce-go plugin");
        let info = serde_json::from_value::<PluginInfo>(info).unwrap();
        let go_bin_path =  String::from("/home/noe/go/bin");
        // let go_bin_path = match env::var("GOBIN") {
        //     Ok(var) => var,
        //     Err(error) => panic!("Problem with GOBIN var: {:?}", error),
        // };
        let file_name = format!("{}/gopls", go_bin_path);

        // if !PathBuf::from(&file_name).exists() {
        //     if cfg!(target_os = "windows") {
        //         Command::new("cmd")
        //                 .args(["/C", "go", "install", "golang.org/x/tools/gopls@latest"])
        //                 .output()
        //                 .expect("failed to execute process")
        //     } else {
        //         Command::new("sh")
        //                 .arg("-c")
        //                 .args(["go", "install", "golang.org/x/tools/gopls@latest"])
        //                 .output()
        //                 .expect("failed to execute process")
        //     };
        // }

        start_lsp(&file_name, "go", info.configuration.options)
    }

}