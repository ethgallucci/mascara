#![warn(unused_crate_dependencies)]
#![allow(dead_code, unused_variables)]

use crate::interface::{DefaultPkg, Packages, FallbackPkg, Config, ConfigStatus};
use std::process::Command;
use std::collections::HashMap;

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
