#![allow(unused)]
#![warn(unused_crate_dependencies)]

use serde::{Deserialize, Serialize};
use std::clone::Clone;
use std::collections::HashMap;
use std::convert::From;
use std::fs;

use crate::bindings::{CliCommand, Flag};

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
pub struct Mascara {
    pub feature: Option<String>,        // feature = "Debian"
    pub fallbacks: Option<Vec<String>>, // fallbacks = ["curl", "sh"]
    pub logs: Option<LogFeatures>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
pub struct DefaultPkg {
    pub cfg: Option<Config>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
pub struct FallbackPkg {
    pub cfg: Option<Config>,
    pub fallback: String,
    pub cmd: Cmd,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
pub struct Packages {
    pub defaults: HashMap<String, DefaultPkg>,
    pub fallbacks: Option<HashMap<String, FallbackPkg>>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
pub struct Manifest {
    pub mascara: Mascara,
    pub packages: Packages,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
pub struct Config {
    // Do this after successfully installalling
    pub after: Option<Cmd>,
    // installallation path
    pub into: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConfigStatus {
    HasAfterNoInto { after: Cmd },
    HasIntoNoAfter { into: String },
    HasBothIntoAfter { into: String, after: Cmd },
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct LogFeatures {
    stdout: String,
    stderr: String
}

impl From<Config> for ConfigStatus {
    fn from(config: Config) -> ConfigStatus {
        match config.after.is_some() {
            true => match config.into.is_some() {
                true => ConfigStatus::HasBothIntoAfter {
                    into: config.into.unwrap().clone(),
                    after: config.after.unwrap().clone(),
                },
                false => ConfigStatus::HasAfterNoInto {
                    after: config.after.unwrap().clone(),
                },
            },
            false => match config.into.is_some() {
                true => ConfigStatus::HasIntoNoAfter {
                    into: config.into.unwrap().clone(),
                },
                false => {
                    panic!("Conf has no status, couldn't convert to ConfigStatus")
                }
            },
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
pub struct Cmd {
    // Binary executable to launch post-installall
    pub bin: String,
    pub args: Option<Vec<String>>,
}

impl Cmd {
    // Collects a Cmd object into Vector
    pub fn collect(self) -> Vec<String> {
        assert!(self.args.is_some());
        self.args.unwrap().clone()
    }
}

impl From<Flag> for Cmd {
    fn from(flag: Flag) -> Self {
        match flag {
            Flag::ConfigAfter { input } => {
                // Split string into vector to parse commands
                let split = input.split(" ");
                let argvec = split.collect::<Vec<&str>>();
                assert!(argvec.len() > 1);

                // Convert to Vec<String>
                let v = argvec
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>();

                Cmd {
                    bin: argvec[0].to_string(),
                    args: Some(v),
                }
            }
            _ => panic!("Failed to convert Flag to Cmd"),
        }
    }
}

pub mod toml_tools {
    use super::*;

    pub fn header_read() -> Mascara {
        // Reading lighter.toml
        let lighter =
            fs::read_to_string("./mascara.toml").expect("failed to read lighter.toml file");
        let raw = format!(r"{lighter}");
        let build: Mascara = toml::from_str(raw.as_str()).unwrap();
        build
    }

    pub fn packages_read() -> Packages {
        let packages = fs::read_to_string("./mascara.toml").expect("failed to read mascara.toml");
        let raw = format!(r"{packages}");
        let pack: Packages = toml::from_str(raw.as_str()).unwrap();
        pack
    }

    pub fn defread() -> Packages {
        let mascara = fs::read_to_string("./mascara.toml").expect("failed to read mascara.toml");
        let raw = format!(r"{mascara}");
        let defaults: Packages = toml::from_str(raw.as_str()).unwrap();
        defaults
    }
}

pub fn split_by_space(s: String) -> Vec<String> {
    let split = s.split(" ");
    let splitvec = split.collect::<Vec<&str>>();
    splitvec
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
}
