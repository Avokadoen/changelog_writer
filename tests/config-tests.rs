#[cfg(test)]
mod config_tests {
    mod file {
        use changelog_writer::config_systems::file::ConfigFile;

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

        #[test]
        fn create_new_config() {
            let json = get_test_json_string();
            match ConfigFile::new(json) {
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
            let config_file = match ConfigFile::new(json) {
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
            let config_file = match ConfigFile::new(json) {
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
            let config_file = match ConfigFile::new(json) {
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
            let config_file = match ConfigFile::new(json) {
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

        #[test]
        fn upgrade_major_with_major_in_file_verified_true() {
            let arg = ArgumentType::Upgrade(String::from("major"));
            let json = file::get_test_json_string();
            let file = match ConfigFile::new(json) {
                Ok(f) => f,
                Err(e) => {
                    panic!("error: {}", e);
                },
            };

            assert!(file_args_merge::verify_arg_to_file_upgrade(arg, file));
        }
        
        #[test]
        fn upgrade_ma_with_major_in_file_verified_true() {
            let arg = ArgumentType::Upgrade(String::from("ma"));
            let json = file::get_test_json_string();
            let file = match ConfigFile::new(json) {
                Ok(f) => f,
                Err(e) => {
                    panic!("error: {}", e);
                },
            };

            assert!(file_args_merge::verify_arg_to_file_upgrade(arg, file));
        }

         #[test]
        fn upgrade_major_with_manure_in_file_verified_false() {
            let arg = ArgumentType::Upgrade(String::from("manure"));
            let json = file::get_test_json_string();
            let file = match ConfigFile::new(json) {
                Ok(f) => f,
                Err(e) => {
                    panic!("error: {}", e);
                },
            };

            assert!(!file_args_merge::verify_arg_to_file_upgrade(arg, file));
        }
    }
}