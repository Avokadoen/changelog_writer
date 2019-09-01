#[cfg(test)]
mod tests {
    use super::*;
    use changelog_writer::ConfigFile;

    fn get_test_json_string() -> String {
        String::from(
        r#"{
            "default_upgrade": "minor",
            "version_types": [ 
                { "major": "Ma" },
                { "minor": "Mi" },
                { "lesser": "Le" }
            ],
            "version_format": "MaMa.MiMi.LeLe",
            "target_file_paths": [ 
                "./something/somthing/changelog.xml", 
                "./something/somthing2/changelog.xml",
                "./changelog.md"
            ],
            "categories": [
                "bugfix",
                "feature",
                "technical",
                "tests"
            ],
            "append_position": "top"
        }"#)
    }

    #[test]
    fn create_new_config() {
        let json = get_test_json_string();
        let config_file = ConfigFile::new(json);
        // this is probably a bad way of doing assert ...
        // alternative but not sure how to print error atm: assert!(config_file.is_ok() && !config_file.is_err());
        match config_file {
            Ok(_) => assert!(true),
            Err(e) => {
                println!("error: {}", e);
                assert!(false)
            },
        }
    }

}