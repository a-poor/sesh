//! App configuration structs.
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    name: String,
    window: Vec<WindowConf>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct WindowConf {
    name: Option<String>,
    command: Option<Vec<String>>,
}
