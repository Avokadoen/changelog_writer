#[cfg(test)]
mod tests {
    use changelog_writer::ConfigFile;

    fn get_test_json_string() -> String {
        // simplefied config for testing
        String::from(
        r#"{
            "default_upgrade": "minor",
            "version_types": [
                { "version_type": [ "major", "Ma" ] },
                { "version_type": [ "minor", "Mi"] },
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
    fn validate_version_types() {
        let json = get_test_json_string();
        let config_file = match ConfigFile::new(json) {
            Ok(c) => c,
            Err(e) => {
                println!("error: {}", e);
                assert!(false)
            },
        }
        match config_file.version_types.get(0) {
            Some(v_type) => println!("The third element is {}", third),
            None => {
                println!("types zero was none!");
                assert!(false);
            }
        }
        assert_eq!()
    }

}