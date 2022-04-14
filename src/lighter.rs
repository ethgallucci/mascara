#![forbid(
    clippy::absurd_extreme_comparisons,
    clippy::suspicious_else_formatting,
    deprecated
)]
#![warn(unused_crate_dependencies, unused_allocation, unused_imports)]

use std::env;

mod interface;
use interface::*;
mod bindings;
use bindings::*;
mod mascara;

fn main() {
    let args: Vec<String> = env::args().collect();
    let clicmd: CliCommand = into_cmd_with_flags(args);
    println!("Parsed cli arguments & flags\n{:?}\n", clicmd.clone());
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn can_deser_mascara() {
        let m = Mascara {
            feature: Some(String::from("Debian")),
            fallbacks: None,
        };

        let deser = toml::to_string(&m).unwrap();
        assert!(!(deser.is_empty()));
        println!("\nDeserialized Mascara: {}", deser)
    }

    #[test]
    fn can_deser_manifest() {
        let m = Mascara{
            feature: Some(String::from("Debian")),
            fallbacks: None,
        };
       
        let def = DefaultPkg {
            cfg: None
        };

        let def1 = DefaultPkg {
            cfg: Some(Config { 
                after: Some(Cmd { bin: "git".to_string(), 
                args: Some(vec!["--version".to_string()]) }), 
            into: None 
            })
        };

        let mut map = HashMap::new();
        map.insert("curl".to_string(), def);
        map.insert("git".to_string(), def1);

        let p = Packages {
            defaults: map, 
            fallbacks: None,
        };

        let manif = Manifest {
            mascara: m,
            packages: p,
        };

        let deser_manifest = toml::to_string(&manif).unwrap();
        println!("\nDeserialized manifest: {}", deser_manifest)
    }

    #[test]
    fn can_ser_manifest() {
        // Read manifest
        let mascara_manifest = std::fs::read_to_string("./mascara.toml").expect("failed to read macara.toml");
        let raw = format!(r"{mascara_manifest}");
        let manifest: Manifest = toml::from_str(raw.as_str()).unwrap();
        println!("Serialized Manifest: {:?}", manifest)
    }

    #[test]
    fn can_read_header() {
        let mascara_env: Mascara = toml_tools::header_read();
        println!("{:?}", mascara_env)
    }
}
