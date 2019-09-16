use std::env;

use changelog_writer::config_systems::*;

fn main() {
    let current_dir = match env::current_dir() {
        Ok(o) => o,
        Err(e) => panic!("failed to read current dir {}", e),
    };
    
    let config_file = match file::ConfigFile::load_config(&current_dir) {
        Ok(o) => o,
        Err(e) => panic!("failed to load config, error: {}", e),
    };

    let args: Vec<String> = env::args().collect();

    let arg_type = match args::ArgumentType::parse_arguments(&args) {
        Ok(o) => o,
        Err(e) => panic!("failed to parse arguments!\nErr: {}", e),
    };
    
    if file_args_merge::verify_arg_to_file_upgrade(arg_type, config_file) {
        panic!("miss-match between config and arguments") // TODO: more precise error message
    }

    print!("hello");    

}

