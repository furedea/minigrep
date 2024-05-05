use std::env;

pub struct CommandLineConfig {
    query: String,
    filename: String,
    case_sensitive: bool,
}

impl CommandLineConfig {
    pub fn from_args(args: &[String]) -> Result<CommandLineConfig, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(CommandLineConfig {
            query,
            filename,
            case_sensitive,
        })
    }

    pub fn query(&self) -> &str {
        &self.query
    }

    pub fn filename(&self) -> &str {
        &self.filename
    }

    pub fn case_sensitive(&self) -> bool {
        self.case_sensitive
    }
}
