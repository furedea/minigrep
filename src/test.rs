use super::*;

#[test]
fn run_file_not_found() {
    let args = vec![
        String::from("minigrep"),
        String::from("foo"),
        String::from("bar"),
    ];
    let config = CommandLineConfig::from_args(&args).unwrap();
    let result = run(config);
    assert!(result.is_err());
}

#[test]
fn run_file_found() {
    env::remove_var("CASE_INSENSITIVE");
    let args = vec![
        String::from("minigrep"),
        String::from("foo"),
        String::from("Cargo.toml"),
    ];
    let config = CommandLineConfig::from_args(&args).unwrap();
    let result = run(config);
    assert!(result.is_ok());
}

#[test]
fn run_case_insensitive() {
    env::set_var("CASE_INSENSITIVE", "1");
    let args = vec![
        String::from("minigrep"),
        String::from("duct"),
        String::from("Cargo.toml"),
    ];
    let config = CommandLineConfig::from_args(&args).unwrap();
    let result = run(config);
    assert!(result.is_ok());
}

#[test]
fn case_sensitive_search() {
    let query = "duct";
    let contents = "\
    Rust:
    safe, fast, productive.
    Pick three.
    Duct tape.";
    assert_eq!(vec!["safe, fast, productive."], search(contents, query));
}

#[test]
fn case_insensitive_search() {
    let query = "rUsT";
    let contents = "\
    Rust:
    safe, fast, productive.
    Pick three.
    Trust me.";
    assert_eq!(
        vec!["Rust:", "Trust me."],
        search_case_insensitive(contents, query)
    );
}

#[test]
fn from_args() {
    env::remove_var("CASE_INSENSITIVE");
    let args = vec![
        String::from("minigrep"),
        String::from("foo"),
        String::from("bar"),
    ];
    let config = CommandLineConfig::from_args(&args).unwrap();
    assert_eq!(config.query(), "foo");
    assert_eq!(config.filename(), "bar");
    assert!(config.case_sensitive());
}

#[test]
fn from_args_not_enough_args() {
    let args = vec![String::from("minigrep")];
    let config = CommandLineConfig::from_args(&args);
    assert!(config.is_err());
}

#[test]
fn from_args_case_insensitive() {
    env::set_var("CASE_INSENSITIVE", "1");
    let args = vec![
        String::from("minigrep"),
        String::from("foo"),
        String::from("bar"),
    ];
    let config = CommandLineConfig::from_args(&args).unwrap();
    assert_eq!(config.query(), "foo");
    assert_eq!(config.filename(), "bar");
    assert!(!config.case_sensitive());
}
