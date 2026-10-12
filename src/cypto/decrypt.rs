use argon2::Argon2;
use chacha20poly1305::{
    XChaCha20Poly1305, XNonce,
    aead::{Aead, KeyInit},
};
use zeroize::Zeroize;

pub fn decrypt_with_password(password: &[u8], payload: &[u8]) -> Result<Vec<u8>, &'static str> {
    if payload.len() < 56 {
        return Err("Payload too short or malformed");
    }

    let salt = &payload[0..16];
    let nonce_bytes = &payload[16..40];
    let ciphertext_with_tag = &payload[40..];

    let mut key = [0u8; 32];
    Argon2::default()
        .hash_password_into(password, salt, &mut key)
        .map_err(|_| "KDF failed")?;

    let cipher = XChaCha20Poly1305::new_from_slice(&key).map_err(|_| "Key error")?;
    let nonce = XNonce::from_slice(nonce_bytes);

    let decrypted = cipher
        .decrypt(nonce, ciphertext_with_tag)
        .map_err(|_| "Decryption failed (wrong password or corrupted data)")?;

    key.zeroize();

    Ok(decrypted)
}
