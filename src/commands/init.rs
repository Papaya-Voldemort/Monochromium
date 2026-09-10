use crate::config::setup_zshrc;

pub async fn init() {
    let prompt_setup = setup_zshrc().await;
}
