//! App configuration structs.
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::fs::{self, read_to_string};
use std::path::PathBuf;
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Validate, Default)]
pub struct Config {
    pub name: String,
    pub window: Vec<WindowConf>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Validate)]
pub struct WindowConf {
    pub name: Option<String>,
    pub command: Option<Vec<String>>,
    #[serde(default)]
    pub default: Option<bool>,
}

impl Config {
    /// Load a config file from path.
    pub fn load(path: &PathBuf) -> Result<Self> {
        if !path.exists() {
            return Err(anyhow!("File does not exist"));
        }
        let txt = read_to_string(&path)?;
        let conf = toml::from_str(&txt)?;
        Ok(conf)
    }

    /// Write a config file to disk
    pub fn write(&self, path: &PathBuf) -> Result<()> {
        let txt = toml::to_string(&self)?;
        fs::write(&path, &txt)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_deserialize_1() -> Result<()> {
        let txt = r#"
# .sesh-conf.toml
name = "my-dir-name" # Current directory name or fun random word combo

[[window]]
name = "editor"
command = ["vim", "."]

[[window]]
name = "claude"
command = ["claude"]

[[window]]
name = "server"
command = ["npm", "run", "dev", "--port", "3000"]
"#;
        let expect = Config {
            name: "my-dir-name".to_string(),
            window: vec![
                WindowConf {
                    name: Some("editor".to_string()),
                    command: Some(vec!["vim".to_string(), ".".to_string()]),
                    default: None,
                },
                WindowConf {
                    name: Some("claude".to_string()),
                    command: Some(vec!["claude".to_string()]),
                    default: None,
                },
                WindowConf {
                    name: Some("server".to_string()),
                    command: Some(vec![
                        "npm".to_string(),
                        "run".to_string(),
                        "dev".to_string(),
                        "--port".to_string(),
                        "3000".to_string(),
                    ]),
                    default: None,
                },
            ],
        };

        let parsed: Config = toml::from_str(txt)?;
        assert_eq!(parsed, expect);
        Ok(())
    }
}
