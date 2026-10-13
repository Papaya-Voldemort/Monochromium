use crate::types::MonoError;
use argon2::Argon2;
use chacha20poly1305::{
    XChaCha20Poly1305, XNonce,
    aead::{Aead, KeyInit},
};
use zeroize::Zeroizing;

pub fn decrypt_with_password(password: &[u8], payload: &[u8]) -> Result<Vec<u8>, MonoError> {
    if payload.len() < 56 {
        return Err(MonoError::Crypto("Payload too short or malformed".into()));
    }

    let salt = &payload[0..16];
    let nonce_bytes = &payload[16..40];
    let ciphertext_with_tag = &payload[40..];

    let mut key = Zeroizing::new([0u8; 32]);
    Argon2::default()
        .hash_password_into(password, salt, &mut *key)
        .map_err(|err| MonoError::Crypto(format!("Key derivation failed: {err}")))?;

    let cipher = XChaCha20Poly1305::new_from_slice(&*key)
        .map_err(|err| MonoError::Crypto(format!("Invalid key: {err}")))?;
    drop(key);

    let nonce = XNonce::try_from(nonce_bytes)
        .map_err(|err| MonoError::Crypto(format!("Invalid nonce: {err}")))?;

    cipher.decrypt(&nonce, ciphertext_with_tag).map_err(|_| {
        MonoError::Crypto("Decryption failed (wrong password or corrupted data)".into())
    })
}

#[cfg(test)]
mod tests {
    use super::decrypt_with_password;
    use crate::{cypto::encrypt_with_password, types::MonoError};

    #[test]
    fn rejects_all_short_payloads() {
        for length in 0..56 {
            assert!(matches!(
                decrypt_with_password(b"password", &vec![0; length]),
                Err(MonoError::Crypto(_))
            ));
        }
    }

    #[test]
    fn rejects_invalid_tag_at_minimum_payload_length() {
        let error = decrypt_with_password(b"password", &[0; 56]).unwrap_err();

        assert!(
            matches!(error, MonoError::Crypto(message) if message.contains("Decryption failed"))
        );
    }

    #[test]
    fn rejects_wrong_password_with_crypto_error() {
        let payload = encrypt_with_password(b"password", b"note").unwrap();

        assert!(matches!(
            decrypt_with_password(b"wrong password", &payload),
            Err(MonoError::Crypto(_))
        ));
    }

    #[test]
    fn rejects_modified_payloads() {
        let payload = encrypt_with_password(b"password", b"note").unwrap();

        for index in [0, 16, 40, payload.len() - 1] {
            let mut modified = payload.clone();
            modified[index] ^= 1;
            assert!(matches!(
                decrypt_with_password(b"password", &modified),
                Err(MonoError::Crypto(_))
            ));
        }

        assert!(matches!(
            decrypt_with_password(b"password", &payload[..payload.len() - 1]),
            Err(MonoError::Crypto(_))
        ));

        let mut extended = payload;
        extended.push(0);
        assert!(matches!(
            decrypt_with_password(b"password", &extended),
            Err(MonoError::Crypto(_))
        ));
    }

    #[test]
    fn roundtrips_empty_and_binary_inputs() {
        for password in [b"".as_slice(), b"password\x00\xff"] {
            for plaintext in [b"".as_slice(), b"note\x00\xff\x80\n"] {
                let payload = encrypt_with_password(password, plaintext).unwrap();

                assert_eq!(payload.len(), 56 + plaintext.len());
                assert_eq!(
                    decrypt_with_password(password, &payload).unwrap(),
                    plaintext
                );
            }
        }
    }
}
