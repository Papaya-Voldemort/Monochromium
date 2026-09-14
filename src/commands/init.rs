use crate::config::setup_zshrc;

pub async fn init() {
    let _prompt_setup = setup_zshrc().await;
}
