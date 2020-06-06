pub fn replace_duplicate(sentence: &str) -> String {
    let mut text_vec = sentence.split_whitespace().collect::<Vec<&str>>();
    text_vec.dedup();
    let words = text_vec.iter().fold(String::new(), |mut s, &text| {
        s.push(' ');
        s.push_str(text);
        s
    });
    String::from(words.trim())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_replace_duplicates() {
        let some_text_with_duplicates = "This is is something";
        assert_eq!(
            replace_duplicate(some_text_with_duplicates),
            "This is something".to_string());
    }
}
