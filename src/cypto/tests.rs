#[cfg(test)]
use crate::cypto::{decrypt::decrypt_with_password, encrypt::encrypt_with_password};
use argon2::{Algorithm, Argon2, Params, Version};

fn fast_test_argon2() -> Argon2<'static> {
    let params = Params::new(8, 1, 1, Some(32)).unwrap();
    Argon2::new(Algorithm::Argon2id, Version::V0x13, params)
}

#[test]
fn roundtrip_encryption() {
    let password = b"password";
    let plaintext = b"Hello, world!";

    let encrypted = encrypt_with_password(password, plaintext).unwrap();
    let decrypted = decrypt_with_password(password, &encrypted).unwrap();

    assert_eq!(decrypted, plaintext);
}

#[test]
fn reject_bad_password() {
    let plaintext = b"Sensitive note content";
    let encrypted = encrypt_with_password(b"password", plaintext).unwrap();

    let result = decrypt_with_password(b"wrong_password", &encrypted);
    assert!(result.is_err());
}
