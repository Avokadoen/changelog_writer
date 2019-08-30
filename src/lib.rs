
#[derive(Debug)]
pub enum VersionType {
    Major,
    Minor,
}

pub struct Config {
    pub version_type: VersionType,
    pub mdPath: String,
    pub htmlPath: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, String> {
        if args.len() < 4 {
            return Err(format!("expected 3 arguments got {}", args.len()));
        }

        let version_type = match args[1].to_lowercase().as_ref() {
            "major" => VersionType::Major,
            "minor" => VersionType::Minor,
            _ => return Err(format!("expected \"major\" or \"minor\" arguments got \"{}\"", args[1].to_lowercase())),
        };

        // TODO: replace clone
        let mdPath = args[2].clone();
        // TODO: replace clone
        let htmlPath = args[3].clone();

        Ok(Config { version_type, mdPath, htmlPath })
    }
}

mod file_modifier {
    fn update(config: Config) {
        
    }

    fn updateMd<'a>(config: Config, contents: &'a str) -> <&'a>String{

    }

    fn updateHtml<'a>(config: Config, contents: &'a str) -> <&'a>String {
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }
}