use argon2::Argon2;
use chacha20poly1305::aead::rand_core::RngCore;
use chacha20poly1305::{
    XChaCha20Poly1305, XNonce,
    aead::{Aead, KeyInit, OsRng},
};
use zeroize::Zeroize;

pub fn encrypt_with_password(password: &[u8], plaintext: &[u8]) -> Result<Vec<u8>, &'static str> {
    let mut salt = [0u8; 16];
    let mut nonce_bytes = [0u8; 24];
    OsRng.fill_bytes(&mut salt);
    OsRng.fill_bytes(&mut nonce_bytes);

    let mut key = [0u8; 32];
    Argon2::default()
        .hash_password_into(password, &salt, &mut key)
        .map_err(|_| "KDF failed")?;

    let cipher = XChaCha20Poly1305::new_from_slice(&key).map_err(|_| "Key error")?;
    let nonce = XNonce::from_slice(&nonce_bytes);
    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|_| "Encryption error")?;

    key.zeroize();

    let mut result = Vec::new();
    result.extend_from_slice(&salt);
    result.extend_from_slice(&nonce_bytes);
    result.extend_from_slice(&ciphertext);

    Ok(result)
}
