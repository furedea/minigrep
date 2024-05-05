use super::*;

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
