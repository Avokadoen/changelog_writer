

pub mod config_systems {
    pub mod file {
        use serde::{Deserialize};

        #[derive(Deserialize)]
        pub struct VersionType {
            pub version_type: [String; 2],
        }

        #[derive(Deserialize)]
        pub struct ConfigFile {
            pub default_upgrade: Option<String>,
            pub version_types: Vec<VersionType>,
            pub version_format: String,
            pub changelog_paths: Vec<String>,
            pub categories: Vec<String>,
            pub append_position: String,
        }

        // TODO: validate values so that they can't be mismatched
        impl ConfigFile {
            pub fn new(json_string: String) -> Result<ConfigFile, Box<dyn std::error::Error + 'static>> {
                //let contents = fs::read_to_string(path)?;
                let config: ConfigFile = serde_json::from_str(&json_string)?;
                Ok(config)
            }
        }
    }
    pub mod args {
        #[derive(PartialEq, Debug)]
        pub enum ArgumentType {
            Init,
            Upgrade,
        }

        impl ArgumentType {
             pub fn parse_arguments(args: &[String]) -> Result<ArgumentType, &'static str> {
                if args.len() != 2 {
                    return Err("expected 1 argument"); // TOOD: got {}
                }
                
                let args_type = match args[1].as_ref() {
                    "-u" | "--upgrade" => ArgumentType::Upgrade,
                    "-i" | "--init" => ArgumentType::Init,
                    _ => return Err("invalid argument"),
                };
            

                Ok(args_type)
            }
        }
    }
}

