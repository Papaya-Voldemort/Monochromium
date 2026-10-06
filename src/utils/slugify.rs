pub fn slugify(input: &str) -> String {
    let slug: String = input
        .chars()
        .map(|c| match c {
            'a'..='z' | '0'..='9' => c,
            'A'..='Z' => c.to_ascii_lowercase(),
            ' ' | '_' | '-' | ':' | '/' | '\\' | ',' | '.' => '-',
            _ => '\0',
        })
        .filter(|&c| c != '\0')
        .collect();

    let parts: Vec<&str> = slug.split('-').filter(|s| !s.is_empty()).collect();
    let result = parts.join("-");

    if result.is_empty() {
        "note".to_string()
    } else {
        result.chars().take(40).collect()
    }
}
