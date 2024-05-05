use std::env::{self, Args};

pub struct CommandLineConfig {
    query: String,
    filename: String,
    case_sensitive: bool,
}

impl CommandLineConfig {
    pub fn from_args(mut args: Args) -> Result<CommandLineConfig, &'static str> {
        args.next();
        let query = get_arg(&mut args, "Didn't get a query string")?;
        let filename = get_arg(&mut args, "Didn't get a filename")?;
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

fn get_arg<T: Iterator<Item = String>>(
    args: &mut T,
    err_msg: &'static str,
) -> Result<String, &'static str> {
    match args.next() {
        Some(arg) => Ok(arg),
        None => Err(err_msg),
    }
}
