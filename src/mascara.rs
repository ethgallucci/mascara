#![warn(unused_crate_dependencies)]
#![allow(dead_code, unused_variables)]

use crate::interface::{DefaultPkg, Packages, FallbackPkg, Config, ConfigStatus};
use std::process::Command;
use std::collections::HashMap;

pub type DMAP = Vec<DefaultPkg>;
pub type TMAP = Vec<String>;

/// Generic Install Errors
#[derive(Debug, PartialEq)]
pub enum InstallErr {
    CmdEmpty,
    CmdTerminated { cmd: crate::interface::Cmd },
    PackEmpty,
}

/// Generic Util
pub mod spark_tools {
    pub fn logproc(proc: std::process::Output) -> () {
        println!("{}", String::from_utf8_lossy(&proc.stdout));
        if !proc.stderr.is_empty() {
            println!("stderr: {}", String::from_utf8_lossy(&proc.stderr));
        }
    }
}

pub mod light_install {
    use super::*;

    /// Generic Light Errors
    #[derive(Debug, Clone, PartialEq)]
    pub enum LightErr {
        RunErr,
        SplitErr,
    }

    /// Collect DefaultPkgs with no cfg
    pub fn collect_no_cfg_defaults(defmap: HashMap<String, DefaultPkg>) -> Result<TMAP, LightErr> {
        let all_keys = defmap.keys();
        let mut no_cfg_keys = vec![];

        for key in all_keys {
            // Get DefaultPkg
            let v = defmap.get_key_value(key).unwrap().1;
            // Check if cfg is empty
            if v.cfg.is_none() { no_cfg_keys.push(key.clone()) }
            else {}
        }

        if no_cfg_keys.is_empty() { Err(LightErr::SplitErr) }
        else { Ok(no_cfg_keys) }
    }

    pub fn exec_light_install_for_debian(target_map: TMAP) -> Result<(), LightErr> {
        for target in target_map {
            let proc = Command::new("sudo")
                .args(["apt-get", "install", target.as_str()])
                .output();
            if proc.is_ok() { spark_tools::logproc(proc.unwrap()) }
        }
        Ok(())
    }

}

/// Full Install
pub mod heavy_install {
    use super::*;
   
    /// Generic Heavy Errors
    #[derive(Debug, PartialEq)]
    pub enum HeavyErr {
        BuildErrMapEmpty,
        DeconstructErr,
    }
    
    // Generic Heavy Success
    #[derive(Clone)]
    pub struct HeavySuccess {}
}
