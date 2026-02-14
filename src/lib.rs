pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}

pub fn case_insensitive_search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    let query = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(query.as_str()) {
            result.push(line);
        }
    }
    result
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn search_case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct Tape.
        ";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }
    #[test]
    fn search_case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
I love Rust.
        ";

        assert_eq!(
            vec!["Rust:", "I love Rust."],
            case_insensitive_search(query, contents)
        )
    }
}
