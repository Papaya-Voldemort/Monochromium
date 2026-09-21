use arboard::Clipboard;

pub fn get_paste() -> Result<String, arboard::Error> {
    let mut clipboard = Clipboard::new()?;

    clipboard.get_text()
}

pub fn copy(text: String) -> Result<(), arboard::Error> {
    let mut clipboard = Clipboard::new()?;

    clipboard.set_text(text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copy_and_get_paste_round_trip() {
        let text = "clipboard test".to_string();

        if let Err(err) = copy(text.clone()) {
            eprintln!("Skipping clipboard test: {err}");
            return;
        }

        let result = match get_paste() {
            Ok(text) => text,
            Err(err) => {
                eprintln!("Skipping clipboard test: {err}");
                return;
            }
        };

        assert_eq!(result, text);
    }
}
