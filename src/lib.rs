fn replace_duplicate(sentence: &str) -> Result<String> {
    let text_vec = sentence.split_whitespace().collect::<Vec<_>>();
    let concatenated_vec_words = text_vec.iter().dedup().fold(String::new(), |mut s, &text| {
        s.push(' ');
        s.push_str(w);
        s
    });
    concatenated_vec_words
}

mod test {
    use super::*;
    fn test_replace_duplicates() {
        let some_text_with_duplicates = "This is is something";
        assert_eq!(
            replace_duplicate(some_text_with_duplicates),
            String::from("This is something")
        );
    }
}
