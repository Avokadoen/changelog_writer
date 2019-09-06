
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
            // TODO: should not hold string as it leads to too much cloning?
            Upgrade(String),
        }

        impl ArgumentType {
             pub fn parse_arguments(args: &[String]) -> Result<ArgumentType, &'static str> {
                if args.len() < 2 || args.len() > 3 {
                    return Err("invalid number of arguments expected 2-3"); // TOOD: got {}
                }
                // TODO: replace clone of string "version"
                let args_type = match args[1].as_ref() {
                    "-u" | "--upgrade" => match args.get(2) {
                       Some(version) => ArgumentType::Upgrade(String::from(version)),
                       None => return Err("upgrade step was none"),
                    }
                    "-i" | "--init" => ArgumentType::Init,
                    _ => return Err("invalid argument"),
                };
            

                Ok(args_type)
            }
        }
    }

    pub mod file_args_merge {
        use super::args::ArgumentType;
        use super::file::ConfigFile;

        pub fn verify_arg_to_file_upgrade(argument_type: ArgumentType, config_file: ConfigFile) -> bool {
            match argument_type {
                ArgumentType::Init => true,
                ArgumentType::Upgrade(s) => 
                    config_file.version_types
                    .iter()
                    .any(|v| v.version_type[0].to_ascii_lowercase() == s || v.version_type[1].to_ascii_lowercase() == s),
            }
        }

    }
}
