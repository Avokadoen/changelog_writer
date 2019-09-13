// TODO: document code as i reach a functional state ///
// TODO: refactor modules
// TODO: create file structure, here and tests!

pub mod config_systems {
    pub mod file {
        use serde::{Deserialize};

        #[derive(Deserialize)]
        pub struct VersionType {
            pub version_type: [String; 2],
        }

        impl VersionType {
            pub fn contains_type(&self, check_type: &str) -> bool {
                return self.version_type[0].to_ascii_lowercase() == check_type || self.version_type[1].to_ascii_lowercase() == check_type;
            }
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
            pub fn new(json_string: &str) -> Result<ConfigFile, Box<dyn std::error::Error + 'static>> {
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

    // TODO: we don't need a mod for one function ...
    pub mod file_args_merge {
        use super::args::ArgumentType;
        use super::file::ConfigFile;

        // TODO: move this to somewhere else (maybe argument type)
        pub fn verify_arg_to_file_upgrade(argument_type: ArgumentType, config_file: ConfigFile) -> bool {
            match argument_type {
                ArgumentType::Init => true,
                ArgumentType::Upgrade(s) => 
                    config_file.version_types
                    .iter()
                    .any(|v| v.contains_type(&s)),
            }
        }
    }

    pub mod changelog_generator {
        use std::fs::File;
        use std::io::prelude::*;
        use std::collections::HashMap;
        use std::path::Path;

        use super::super::git_data_fetcher;

        pub fn create_changelog(path: &str, content: &[u8]) -> Result<(), &'static str> {
        
            let mut file = match File::create(path) {
                Ok(o) => o,
                Err(_) => return Err("failed to create file"),
            };
            match file.write_all(content) {
                Err(_) => return Err("failed to write bytes to file"),
                _ => (),
            };
            Ok(())
        }

        // TODO: refactor, and just use paramateres supplied as we have to do a lot of extra heap stuff with HashMap
        pub fn parse_commit_msgs_to_md(msgs: Vec<git_data_fetcher::CommitMessageLog>, config_categories: Vec<String>, new_version: &str) -> String {
            // create arrays or vecs according to config categories len
            let mut changelog_msgs: HashMap<String, String> = HashMap::new();

            for msg in msgs {
                if config_categories.contains(&msg.category) {
                    changelog_msgs.insert(msg.category, msg.msg);
                } else {
                    eprint!("found invalid category: {} with message {}\ncheck your config file", msg.category, msg.msg);
                }
            }

            let mut new_version_changelog_md = String::from(format!("## {}\n", new_version));

            for cat in changelog_msgs.keys() {
                new_version_changelog_md.push_str(&format!("\n   #### {}", cat));

                for msg in changelog_msgs.get(cat) {
                    new_version_changelog_md.push_str(&format!("\n      - {}", msg));
                }
                new_version_changelog_md.push('\n');
            }

            return new_version_changelog_md;
        }

        pub fn insert_parsed_commits(parsed_commits: &str, path: &Path) {
            
        }
    }
}

pub mod git_data_fetcher {
    use std::path::Path;
    use std::fs::{self};
    
    pub struct CommitMessageLog {
        pub category: String,
        pub msg: String,
    }

    impl CommitMessageLog {
        pub fn new_from_vars(set_cat: &str, set_msg: &str) -> CommitMessageLog {
            CommitMessageLog {
                category: String::from(set_cat),
                msg: String::from(set_msg),
            }
        }

        pub fn new_from_line(line: &str) -> Option<CommitMessageLog> {
            let mut event_iter = line.split("cat:");
            event_iter.next();

            let filtered_string = match event_iter.next() {
                Some(o) => String::from(o),
                None => return None,
            };

            let category_value = match filtered_string.split(" ").next() {
                Some(o) => String::from(o),
                None => return None,
            };

            let mut msg_iterator = filtered_string.split("'");
            msg_iterator.next();
            let msg_value = match msg_iterator.next() {
                Some(o) => String::from(o),
                None => return None,
            };

            Some(CommitMessageLog {
                category: category_value,
                msg: msg_value,
            })
        }
    }

    // TODO: refactor, too much heap
    pub fn create_commit_msgs_to_parse(prev_line: usize, git_dir: &Path) -> Result<Vec<CommitMessageLog>, &'static str> {
        let logs_dir = git_dir.join("logs");

        if !logs_dir.is_dir() {
            eprintln!("logs_dir: {}", logs_dir.into_os_string().into_string().unwrap());
            return Err("logs dir was not a dir");
        }

        let head_path = logs_dir.join("HEAD");
        if !head_path.is_file() {
            return Err("head path was not file");
        }

        let head_content = match fs::read_to_string(head_path) {
            Ok(s) => s,
            Err(_) => return Err("failed to read head"),
        };

        let commit_events = head_content.split("\n").collect::<Vec<&str>>();

        if commit_events.len() < prev_line {
            return Err("invalid prev_line value");
        }

        // take collection iterator, convert to vec and then slice vec to just relevant content 
        let relevant_event = &commit_events[prev_line..];

        // TODO: refactor this to more function oriented (check if there are some proper functions to get needed iterators)
        let mut new_msgs: Vec<CommitMessageLog> = Vec::new(); 
        'events: for event in relevant_event {
            new_msgs.push(match CommitMessageLog::new_from_line(event) {
                Some(m) => m,
                None => continue 'events,
            });
        }

        // TODO: call validate?

        Ok(new_msgs)
    }

    pub fn validate_commit_msgs(new_msgs: Vec<CommitMessageLog>, config_categories: Vec<String>) -> Result<(), String> {
        for msg in new_msgs {
            if !config_categories.contains(&msg.category) {
                return Err(format!("invalid category {}", msg.category))
            }
        }

        Ok(())
    }
}