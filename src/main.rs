use std::env;
use std::path::Path;

use changelog_writer::config_systems::*;
use changelog_writer::changelog_generator;
use changelog_writer::git_data_fetcher;

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
    
    if !file_args_merge::verify_arg_to_file_upgrade(&arg_type, &config_file) {
        panic!("miss-match between config and arguments") // TODO: more precise error message
    }

    let prev_line = match arg_type {
        args::ArgumentType::Init => 0,
        args::ArgumentType::Upgrade(_) => 0, // function to find this in file
    };

    // TODO: we should use some sort of find to solve .git dir just like we should with config
    let commit_msgs = match git_data_fetcher::create_commit_msgs_to_parse(prev_line, &current_dir.join(".git")) {
        Ok(m) => m,
        Err(e) => panic!("\nfailed to retrieve changes in git, error: {}\n", e),
    };
    // TODO upgrade step, only if vec contains any .md
    let md_changes = changelog_generator::parse_commit_msgs_to_md(commit_msgs, config_file.categories, "0.0.0"); 
    for changelog in config_file.changelog_paths {
        if changelog.contains(".md") {
            if let Err(e) = changelog_generator::create_changelog(&md_changes, &Path::new(&changelog)) {
                panic!("failed to write changelog\nError: {}", e);
            }
        }
    }

    println!("Changelogs created");
}

