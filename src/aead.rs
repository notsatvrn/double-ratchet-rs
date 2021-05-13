use aes_gcm_siv::{Key, Aes256GcmSiv, Nonce};
use aes_gcm_siv::aead::{NewAead, AeadInPlace};
use alloc::vec::Vec;

const CONSTANT_NONCE: &[u8] = b"Super Noncel";

pub fn encrypt(mk: &[u8; 32], plaintext: &[u8], associated_data: &[u8]) -> Vec<u8> {
    let key = Key::from_slice(mk);
    let cipher = Aes256GcmSiv::new(key);

    let nonce = Nonce::from_slice(&CONSTANT_NONCE);
    let mut buffer = Vec::new();
    buffer.extend_from_slice(plaintext);

    cipher.encrypt_in_place(nonce, associated_data, &mut buffer)
        .expect("Encryption failed");
    buffer
}

pub fn decrypt(mk: &[u8; 32], ciphertext: &[u8], associated_data: &[u8]) -> Vec<u8> {
    let key = Key::from_slice(mk);
    let cipher = Aes256GcmSiv::new(key);

    let nonce = Nonce::from_slice(&CONSTANT_NONCE);
    let mut buffer = Vec::new();
    buffer.extend_from_slice(ciphertext);
    cipher.decrypt_in_place(nonce, associated_data, &mut buffer).expect("Decryption failure");
    buffer
}

#[cfg(test)]
mod tests {
    use crate::kdf_chain::gen_mk;
    use crate::aead::{encrypt, decrypt};

    #[test]
    fn enc_a_dec() {
        let test_data = include_bytes!("aead.rs").to_vec();
        let associated_data = include_bytes!("lib.rs").to_vec();
        let mk = gen_mk();
        let ciphertext = encrypt(&mk, &test_data, &associated_data);
        let plaintext = decrypt(&mk, &ciphertext, &associated_data);
        assert_eq!(test_data, plaintext)
    }
}