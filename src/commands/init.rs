use crate::config::setup_zshrc;

pub async fn init() {
    match setup_zshrc().await {
        Ok(_) => println!("Successfully configured Monochromium reminders in ~/.zshrc!"),
        Err(e) => eprintln!("Failed to configure zshrc: {}", e),
    }
}
