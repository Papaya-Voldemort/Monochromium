// Build out tiny ML model to do text -> title
pub fn make_title(text: String) -> String {
    let words: Vec<&str> = text.split_whitespace().take(5).collect();

    let result = words.join(" ");

    result
}
