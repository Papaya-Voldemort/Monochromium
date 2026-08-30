use arboard::Clipboard;

pub fn paste() -> String {
    let mut clipboard = Clipboard::new().unwrap();

    clipboard.get_text().unwrap()
}

pub fn copy(text: String) {
    let mut clipboard = Clipboard::new().unwrap();

    clipboard.set_text(text).unwrap();
}