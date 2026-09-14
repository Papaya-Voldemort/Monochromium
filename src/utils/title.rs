// Build out tiny ML model to do text -> title
pub fn make_title(text: String) -> String {
    let words: Vec<&str> = text.split_whitespace().take(5).collect();

    words.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_title_truncates_at_five_words() {
        let title = make_title("one two three four five six seven".to_string());
        assert_eq!(title, "one two three four five");
    }
}