use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CliCommand {
    // Add a package to lighter.toml
    Add {
        pkg: String,
        flags: Option<Vec<Flag>>,
    },
    // Execute installation and configuration
    Spark,
    // Dry run to verify lighter.toml is properly formatted
    Check,
    // Run config scripts only (no new installs)
    Config,
    Unknown,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Flag {
    // Apply this flag at the end of 'add' Clicommands to specify post-installation configs
    ConfigAfter { input: String },
    // Apply this flag at the end of 'add' Clicommands to set a custom path for installs
    Into { path: String },
    NoFlags,
}

pub fn into_flag(args: Vec<String>) -> Result<Flag, ()> {
    match &args[3] as &str {
        "--config-after" => {
            if args.iter().len() < 5 {
                Err(())
            } else {
                Ok(Flag::ConfigAfter {
                    input: args[4].clone(),
                })
            }
        }
        "--into" => {
            if args.iter().len() < 5 {
                Err(())
            } else {
                Ok(Flag::Into {
                    path: args[4].clone(),
                })
            }
        }
        _ => Err(()),
    }
}

pub fn usage() {
    println!("lighter\n\nClicommands\nadd\nspark\ncheck\nconfig\n\n")
}

pub fn into_cmd_with_flags(args: Vec<String>) -> CliCommand {
    if args.iter().len() > 1 {
        match &args[1] as &str {
            "add" => {
                if args.len() < 4 {
                    CliCommand::Add {
                        pkg: args[2].clone(),
                        flags: None,
                    }
                } else {
                    let flag = into_flag(args.clone()).unwrap_or(Flag::NoFlags);
                    CliCommand::Add {
                        pkg: args[2].clone(),
                        flags: Some(vec![flag]),
                    }
                }
            }
            "spark" => CliCommand::Spark,
            "check" => CliCommand::Check,
            "config" => CliCommand::Config,
            _ => CliCommand::Unknown,
        }
    } else {
        usage();
        CliCommand::Unknown
    }
}

