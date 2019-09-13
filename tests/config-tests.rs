#[cfg(test)]
mod config_tests {

    // move to some test utlity module
    pub fn get_test_json_string() -> String {
        // simplefied config for testing
        String::from(
        r#"{
            "default_upgrade": "minor",
            "version_types": [
                { "version_type": [ "major", "Ma" ] },
                { "version_type": [ "minor", "Mi"] }
            ], 
            "version_format": "MaMa.MiMi",
            "changelog_paths": [ 
                "./changelogs/somthing1/changelog.xml", 
                "./changelogs/somthing2/changelog.xml",
                "./changelogs/changelog.md"
            ],
            "categories": [
                "bugfix",
                "feature",
                "tests"
            ],
            "append_position": "top"
        }"#)
    }

    // rename this to something more sane
    mod file {
        use changelog_writer::config_systems::file::ConfigFile;
        use super::*;

        #[test]
        fn create_new_config() {
            let json = get_test_json_string();
            match ConfigFile::new(&json) {
                Ok(_) => assert!(true),
                Err(e) => {
                    println!("error: {}", e);
                    assert!(false)
                },
            }
        }

        #[test]
        fn validate_default_upgrade_minor() {
            let json = get_test_json_string();
            let config_file = match ConfigFile::new(&json) {
                Ok(c) => c,
                Err(e) => {
                    panic!("error: {}", e);
                },
            };
            match config_file.default_upgrade {
                Some(default_version) => {
                    assert_eq!(default_version, "minor");
                }
                None => {
                    panic!("types 0 was none!");
                }
            };
        }

        #[test]
        fn validate_version_types_first_element_major() {
            let json = get_test_json_string();
            let config_file = match ConfigFile::new(&json) {
                Ok(c) => c,
                Err(e) => {
                    panic!("error: {}", e);
                },
            };
            match config_file.version_types.get(0) {
                Some(v_type) => {
                    assert_eq!(v_type.version_type[0], "major");
                }
                None => {
                    panic!("types 0 was none!");
                }
            };
        }

        #[test]
        fn validate_version_types_last_element_minor() {
            let json = get_test_json_string();
            let config_file = match ConfigFile::new(&json) {
                Ok(c) => c,
                Err(e) => {
                    panic!("error: {}", e);
                },
            };
            match config_file.version_types.get(1) {
                Some(v_type) => {
                    assert_eq!(v_type.version_type[0], "minor");
                }
                None => {
                    panic!("types 1 was none!");
                }
            };
        }


        #[test]
        fn validate_version_format_mamamimi() {
            let json = get_test_json_string();
            let config_file = match ConfigFile::new(&json) {
                Ok(c) => c,
                Err(e) => {
                    panic!("error: {}", e);
                },
            };
            match config_file.changelog_paths.get(2) {
                Some(path) => {
                    assert_eq!(path, "./changelogs/changelog.md");
                }
                None => {
                    panic!("path was none!");
                }
            };
        }
    }
    
    mod args {
        use changelog_writer::config_systems::args::ArgumentType;
        // TODO: use str constants instead if possible
        #[test]
        fn too_many_args_result_in_err() {
            assert!(ArgumentType::parse_arguments(&[String::from("ignored"), String::from("ok"), String::from("too_many")]).is_err());
        }

        #[test]
        fn invalid_arg_result_in_err() {
            assert!(ArgumentType::parse_arguments(&[String::from("ignored"), String::from("illegal")]).is_err());
        }

        #[test]
        fn no_upgrade_step_on_upgrade_result_in_err() {
            assert!(ArgumentType::parse_arguments(&[String::from("ignored"), String::from("-u")]).is_err());
        }
    
        #[test]
        fn valid_arg_result_in_ok() {
            assert!(ArgumentType::parse_arguments(&[String::from("ignored"), String::from("-u"), String::from("none valid but not validated yet")]).is_ok());
        }

        #[test]
        fn dash_u_arg_result_in_upgrade() {
            let argument_type = match ArgumentType::parse_arguments(&[String::from("ignored"), String::from("-u"), String::from("major")]) {
                Ok(t) => t,
                Err(_) => panic!("got error on -u"),
            };
            assert_eq!(argument_type, ArgumentType::Upgrade(String::from("major")));
        }

        #[test]
        fn dash_upgrade_arg_result_in_upgrade() {
            let argument_type = match ArgumentType::parse_arguments(&[String::from("ignored"), String::from("--upgrade"), String::from("major")]) {
                Ok(t) => t,
                Err(_) => panic!("got error on --upgrade"),
            };
            assert_eq!(argument_type, ArgumentType::Upgrade(String::from("major")));
        }

        #[test]
        fn dash_i_arg_result_in_upgrade() {
            let argument_type = match ArgumentType::parse_arguments(&[String::from("ignored"), String::from("-i")]) {
                Ok(t) => t,
                Err(_) => panic!("got error on -i"),
            };
            assert_eq!(argument_type, ArgumentType::Init);
        }

        #[test]
        fn dash_init_arg_result_in_upgrade() {
            let argument_type = match ArgumentType::parse_arguments(&[String::from("ignored"), String::from("--init")]) {
                Ok(t) => t,
                Err(_) => panic!("got error on --init"),
            };
            assert_eq!(argument_type, ArgumentType::Init);
        }
    }

    mod file_args_merge {
        use super::*;
        use changelog_writer::config_systems::file::ConfigFile;
        use changelog_writer::config_systems::args::ArgumentType;
        use changelog_writer::config_systems::file_args_merge;

        fn setup_tests(arg_string: String) -> (ArgumentType, ConfigFile) {
            let json = get_test_json_string();
            let file = match ConfigFile::new(&json) {
                Ok(f) => f,
                Err(e) => {
                    panic!("error: {}", e);
                },
            };
            let arg = ArgumentType::Upgrade(arg_string);

            (arg, file)
        }

        #[test]
        fn upgrade_major_with_major_in_file_verified_true() {
            let (arg, file) = setup_tests(String::from("major"));

            assert!(file_args_merge::verify_arg_to_file_upgrade(arg, file));
        }
        
        #[test]
        fn upgrade_ma_with_major_in_file_verified_true() {
            let (arg, file) = setup_tests(String::from("ma"));

            assert!(file_args_merge::verify_arg_to_file_upgrade(arg, file));
        }

        #[test]
        fn upgrade_minor_with_major_in_file_verified_true() {
            let (arg, file) = setup_tests(String::from("minor"));

            assert!(file_args_merge::verify_arg_to_file_upgrade(arg, file));
        }

        #[test]
        fn upgrade_mi_with_major_in_file_verified_true() {
            let (arg, file) = setup_tests(String::from("mi"));

            assert!(file_args_merge::verify_arg_to_file_upgrade(arg, file));
        }


        #[test]
        fn upgrade_major_with_manure_in_file_verified_false() {
            let (arg, file) = setup_tests(String::from("manure"));

            assert!(!file_args_merge::verify_arg_to_file_upgrade(arg, file));
        }
    }

    mod changelog_generator {
        use changelog_writer::config_systems::changelog_generator;
        use changelog_writer::git_data_fetcher;
        use std::fs;

        fn cleanup_file(path: &str) {
            match fs::remove_file(path) {
                Ok(_) => (),
                Err(_) => panic!("failed to delete test file {}", path),
            }
        }

        // TODO: these tests should use match as ok result in file that needs cleanup, and err does not
        #[test]
        fn init_changelog_md_results_ok() {
            let test_ok = changelog_generator::create_changelog("hello_world.md", b"test").is_ok();
            if test_ok {
                cleanup_file("hello_world.md");
            }
            assert!(test_ok);
        }

        #[test]
        fn parse_commit_creates_valid_output() {
            let msgs: Vec<git_data_fetcher::CommitMessageLog> = vec![
                git_data_fetcher::CommitMessageLog::new_from_vars("maintainfeat", "did some readme stuff maybe"),
                git_data_fetcher::CommitMessageLog::new_from_vars("tests", "created 1000th unit test"),
            ];

            let categories: Vec<String> = vec![String::from("maintainfeat"), String::from("tests")];

            let md_changes = changelog_generator::parse_commit_msgs_to_md(msgs, categories, "1.1.1");

            // we cant assert the full string as it uses a hashmap which has random access order
            if !&md_changes.contains("#### maintainfeat\n      - did some readme stuff maybe\n") {
                assert!(false, "maintainstring was of unexpected value");
            }
            if !&md_changes.contains("#### tests\n      - created 1000th unit test\n") {
                assert!(false, "tests was of unexpected value");
            }
            if !&md_changes.contains("## 1.1.1\n\n") {
                assert!(false, "version was of unexpected value");
            }

            assert!(true);
        }

    }

    mod git_data_fetcher {
        use changelog_writer::git_data_fetcher;

        use std::path::Path;

        #[test]
        fn create_commit_msgs_from_test_file() {
            match git_data_fetcher::create_commit_msgs_to_parse(0, Path::new("./tests/resources/.git_mock/")) 
                {
                    Ok(_) => assert!(true),
                    Err(e) => panic!("error: {}", e),
                }
        }

        #[test]
        fn fetch_from_19_gives_1_msg() {
            match git_data_fetcher::create_commit_msgs_to_parse(19, Path::new("./tests/resources/.git_mock/")) 
                {
                    Ok(o) => {if o.len() == 1 {
                        assert!(true)
                    } else {
                        println!("vec lenght excpected to be 1 was {}", o.len());
                        assert!(false)
                    }
                    },
                    Err(e) => panic!("error: {}", e),
                }
        }

        #[test]
        fn valid_categories_to_msg_relation() {
            let msgs: Vec<git_data_fetcher::CommitMessageLog> = vec![
                git_data_fetcher::CommitMessageLog::new_from_vars("maintainfeat", ""),
                git_data_fetcher::CommitMessageLog::new_from_vars("tests", ""),
            ];

            let categories: Vec<String> = vec![String::from("maintainfeat"), String::from("tests")];

            assert!(git_data_fetcher::validate_commit_msgs(msgs, categories).is_ok());   
        }

        #[test]
        fn invalid_categories_to_msg_relation() {
            let msgs: Vec<git_data_fetcher::CommitMessageLog> = vec![
                git_data_fetcher::CommitMessageLog::new_from_vars("maintainfeat", ""),
                git_data_fetcher::CommitMessageLog::new_from_vars("tests", ""),
            ];

            let categories: Vec<String> = vec![String::from("tests")];

            assert!(git_data_fetcher::validate_commit_msgs(msgs, categories).is_err());  
        }
    }
}


    