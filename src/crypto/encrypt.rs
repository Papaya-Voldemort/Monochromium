use crate::crypto::argon::argon2_config;
use crate::types::MonoError;
use chacha20poly1305::{
    XChaCha20Poly1305, XNonce,
    aead::{Aead, KeyInit},
};
use zeroize::Zeroizing;

pub fn encrypt_with_password(password: &[u8], plaintext: &[u8]) -> Result<Vec<u8>, MonoError> {
    let mut salt = [0u8; 16];
    let mut nonce_bytes = [0u8; 24];
    getrandom::fill(&mut salt)?;
    getrandom::fill(&mut nonce_bytes)?;

    let mut key = Zeroizing::new([0u8; 32]);

    argon2_config()
        .hash_password_into(password, &salt, &mut *key)
        .map_err(|err| MonoError::Crypto(format!("Key derivation failed: {err}")))?;

    let cipher = XChaCha20Poly1305::new_from_slice(&*key)
        .map_err(|err| MonoError::Crypto(format!("Invalid key: {err}")))?;
    drop(key);

    let nonce = XNonce::try_from(nonce_bytes.as_slice())
        .map_err(|err| MonoError::Crypto(format!("Invalid nonce: {err}")))?;
    let ciphertext = cipher
        .encrypt(&nonce, plaintext)
        .map_err(|_| MonoError::Crypto("Encryption failed".into()))?;

    let mut result = Vec::new();
    result.extend_from_slice(&salt);
    result.extend_from_slice(&nonce_bytes);
    result.extend_from_slice(&ciphertext);

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::encrypt_with_password;

    #[test]
    fn uses_fresh_salt_and_nonce() {
        let first = encrypt_with_password(b"password", b"note").unwrap();
        let second = encrypt_with_password(b"password", b"note").unwrap();

        assert_eq!(first.len(), 56 + b"note".len());
        assert_eq!(second.len(), first.len());
        assert_ne!(&first[..16], &second[..16]);
        assert_ne!(&first[16..40], &second[16..40]);
    }
}
