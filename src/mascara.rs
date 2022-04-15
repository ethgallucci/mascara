#![warn(unused_crate_dependencies)]
#![allow(dead_code, unused_variables, unused_assignments)]

use crate::interface::{
    Cmd, Config, ConfigStatus, DefaultPkg, FallbackPkg, Manifest, Mascara, Packages,
};
use std::collections::HashMap;
use std::process::Command;

use colored::Colorize;

pub type DMAP = Vec<DefaultPkg>;
pub type TMAP = Vec<String>;

/// Generic Install Errors
#[derive(Debug, PartialEq)]
pub enum InstallErr {
    CmdEmpty,
    CmdTerminated { cmd: Cmd },
    PackEmpty,
}

/// Generic Util
pub mod mascara_util {
    use super::*;

    pub fn build_heavy_dmap(defpkgs: HashMap<String, DefaultPkg>) -> Result<DMAP, ParseErr> {
        let mut dmap = vec![];
        for pkg in defpkgs.values() {
            dmap.push(pkg.clone())
        }
        Ok(dmap)
    }

    pub fn build_heavy_tmap_default(dpkgs: HashMap<String, DefaultPkg>) -> Result<TMAP, ParseErr> {
        let mut tmap = vec![];
        for target in dpkgs.keys() {
            tmap.push(target.clone())
        }
        Ok(tmap)
    }

    pub fn logproc(proc: std::process::Output) -> () {
        if !proc.stderr.is_empty() {
            let err = format!("ERR: {}", String::from_utf8_lossy(&proc.stderr)).red();
            println!("{}", err)
        }

        let output = format!("{}", String::from_utf8_lossy(&proc.stdout))
            .bold()
            .blue();
        println!("{}", output)
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Feature {
        Arch,
        Darwin,
        Debian,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum ParseErr {
        DiscernFeatureErr,
    }

    pub fn discern_feature(header: Mascara) -> Result<Feature, ParseErr> {
        // [TODO]
        // If feature is none we need to use system details to determine which pm to use
        assert!(header.feature.is_some());
        let feature = header.feature.unwrap();

        match feature.as_str() {
            "Arch" => Ok(Feature::Arch),
            "Darwin" => Ok(Feature::Darwin),
            "Debian" => Ok(Feature::Debian),
            _ => Err(ParseErr::DiscernFeatureErr),
        }
    }

    pub fn serialize_mascara_manifest() -> Result<Manifest, ParseErr> {
        let manifest =
            std::fs::read_to_string("./mascara.toml").expect("failed to read mascara.toml");
        let raw = format!(r"{manifest}");
        let serialized_manifest: Manifest = toml::from_str(raw.as_str()).unwrap();
        Ok(serialized_manifest)
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
            if v.cfg.is_none() {
                no_cfg_keys.push(key.clone())
            } else {
            }
        }

        if no_cfg_keys.is_empty() {
            Err(LightErr::SplitErr)
        } else {
            Ok(no_cfg_keys)
        }
    }

    pub fn exec_light_install_for_debian(target_map: TMAP) -> Result<(), LightErr> {
        for target in target_map {
            let proc = Command::new("sudo")
                .args(["apt-get", "install", target.as_str()])
                .output();
            if proc.is_ok() {
                mascara_util::logproc(proc.unwrap())
            }
        }
        Ok(())
    }
}

/// Full Install
pub mod heavy_install {
    use super::*;
    use mascara_util::Feature;

    /// Generic Heavy Errors
    #[derive(Debug, PartialEq)]
    pub enum HeavyErr {
        BuildErrMapEmpty,
        CfgErr,
        DeconstructErr,
        Unimplemented,
    }

    // Generic Heavy Success
    #[derive(Clone)]
    pub struct HeavySuccess {}

    pub type CFGMAP = Vec<ConfigStatus>;

    pub fn perform_cfg_after(after: Cmd) -> Result<(), HeavyErr> {
        let bin = after.bin.clone();
        let cfgproc = Command::new(bin)
            .args([after.args.unwrap().concat().as_str()])
            .output()
            .expect("Failed at perform_cfg_after");
        mascara_util::logproc(cfgproc);
        Ok(())
    }

    // Routes from feature to correct function
    pub fn handle_feature_for_default(
        header: Mascara,
        dmap: DMAP,
        tmap: TMAP,
    ) -> Result<HeavySuccess, HeavyErr> {
        let feature = mascara_util::discern_feature(header).unwrap();
        match feature {
            Feature::Arch => {
                unimplemented!()
            }
            Feature::Darwin => {
                unimplemented!()
            }
            Feature::Debian => Ok(perform_default_with_cfg_for_debian(dmap, tmap).unwrap()),
        }
    }

    pub fn perform_default_with_cfg_for_debian(
        def_map: DMAP,
        target_map: TMAP,
    ) -> Result<HeavySuccess, HeavyErr> {
        let mut i = 0;
        while (i < target_map.len()) {
            let curr_has_cfg: bool;

            let curr_target = target_map[i].clone();
            let curr_dpkg_cfg = def_map[i].clone().cfg;

            // Try Target
            let try_for_target = Command::new("sudo")
                .args(["apt-get", "install", curr_target.as_str()])
                .output()
                .expect("failed to fetch target");
            mascara_util::logproc(try_for_target);

            if curr_dpkg_cfg.is_none() {
                curr_has_cfg = false
            } else {
                curr_has_cfg = true;
                let curr_cfg = curr_dpkg_cfg.unwrap();
                let curr_after = curr_cfg.after.unwrap();
                let curr_args = curr_after.args.unwrap();

                // Try CFG
                let try_cfg_after = Command::new(curr_after.bin)
                    .args(curr_args.iter().map(|a| a.as_str()))
                    .output()
                    .expect("failed at cfg_after");
                mascara_util::logproc(try_cfg_after);
            }

            i += 1;
        }
        Ok(HeavySuccess {})
    }
}
