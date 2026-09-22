// Build out tiny ML model to do text -> title
pub fn trim_text(text: String, limit: usize) -> String {
    let words: Vec<&str> = text.split_whitespace().take(limit).collect();

    words.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trim_text_truncates() {
        let title = trim_text("one two three four five six seven".to_string(), 5);
        assert_eq!(title, "one two three four five");
    }
}
