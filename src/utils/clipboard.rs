use arboard::Clipboard;

pub fn get_paste() -> Result<String, arboard::Error> {
    let mut clipboard = Clipboard::new()?;

    clipboard.get_text()
}

pub fn copy(text: String) -> Result<(), arboard::Error>{
    let mut clipboard = Clipboard::new()?;

    clipboard.set_text(text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copy_and_get_paste_round_trip() -> Result<(), arboard::Error> {
        let text = "clipboard test".to_string();

        copy(text.clone())?;
        let result = get_paste()?;

        assert_eq!(result, text);

        Ok(())
    }
}
