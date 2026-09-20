use crate::config::setup_zshrc;

pub fn init() {
    match setup_zshrc() {
        Ok(_) => println!("Successfully configured Monochromium reminders in ~/.zshrc!"),
        Err(e) => eprintln!("Failed to configure zshrc: {}", e),
    }
}
