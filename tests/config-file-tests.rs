#[cfg(test)]
mod tests {
    use super::*;
    use changelog_writer::ConfigFile;
    use std::path::Path;

    #[test]
    fn create_new_config() {
        let config_path = Path::new("/tests/config-file-tests.rs");
        println!("path: {:?}", config_path);
        let config_file = ConfigFile::new(config_path);
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