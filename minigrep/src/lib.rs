pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = vec![];

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = vec![];

    for line in contents.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            results.push(line);
        }
    }

    results
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
        assert_eq!(search(query, contents), vec!["safe, fast, productive."]);
    }

    #[test]
    fn two_result_case_sensitive() {
        let query = "bUsT";
        let empty_result: Vec<&str> = Vec::new();
        let contents = "\
Bust:
safe, fast productive.
Pick three.
Don't Bust me.";

        assert_eq!(empty_result, search(query, contents));
    }

    #[test]
    fn two_result_case_insensitive() {
        let query = "bUsT";
        let contents = "\
Bust:
safe, fast productive.
Pick three.
Don't Bust me.";

        assert_eq!(
            vec!["Bust:", "Don't Bust me."],
            search_case_insensitive(query, contents)
        );
    }
}
