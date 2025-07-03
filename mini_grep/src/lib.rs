pub fn search<'a>(key_word: &'a str, content: &'a str) -> Vec<&'a str> {
    content.lines().filter(|x| x.contains(key_word)).collect()
}

pub fn search_case_insensitive<'a>(key_word: &'a str, content: &'a str) -> Vec<&'a str> {
    let query = key_word.to_ascii_lowercase();
    content
        .lines()
        .filter(|x| x.to_ascii_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_case_1() {
        let query = "Rust";
        let contents = "Rust is great\nrust is fast\nC++ is fast";
        assert_eq!(search(query, contents), vec!["Rust is great"]);
    }

    #[test]
    fn test_case_2() {
        let query = "rust";
        let contents = "Rust is great\nrust is fast\nC++ is fast";
        let str = search_case_insensitive(query, contents);
        assert_eq!(str, vec!["Rust is great", "rust is fast"]);
    }
}
