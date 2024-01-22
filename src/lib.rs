/*
encryptodon is a goofy tool for e2ee 1:1 communications using just strings

Copyright (C) 2024 sean watters

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as
published by the Free Software Foundation, either version 3 of the
License, or (at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

use wasm_bindgen::prelude::*;

use aes::{
    self,
    cipher::{generic_array::GenericArray, BlockDecrypt, BlockEncrypt, KeyInit},
};
use base64::{engine::general_purpose::STANDARD as b64, Engine};

fn string_keys_to_shared_secret(pub_key: &str, priv_key: &str) -> Result<[u8; 32], String> {
    let pub_key_as_bytes: [u8; 32] = match match b64.decode(pub_key) {
        Ok(k) => k,
        Err(_) => return Err("failed to decode pub key".to_string()),
    }
    .try_into()
    {
        Ok(k) => k,
        Err(_) => return Err("failed to convert decoded pub key to fixed bytes".to_string()),
    };

    let priv_key_as_bytes: [u8; 32] = match match b64.decode(priv_key) {
        Ok(k) => k,
        Err(_) => return Err("failed to decode priv key".to_string()),
    }
    .try_into()
    {
        Ok(k) => k,
        Err(_) => return Err("failed to convert decoded priv key to fixed bytes".to_string()),
    };

    let pub_key = x25519_dalek::PublicKey::from(pub_key_as_bytes);
    let priv_key = x25519_dalek::StaticSecret::from(priv_key_as_bytes);

    Ok(priv_key.diffie_hellman(&pub_key).to_bytes())
}

/// pulls their public key from bio.
///
/// ```rust
/// let bio = "i like dogs and computers\n 🐘🔑:0bmKKWS04lZzoPC/KlS1kJgWN+XnvBw0yn4PPnot73E=\n more stuff...".to_string();
///
/// let key = encryptodon::extract_key_from_bio(bio).unwrap();
///
/// assert_eq!(key, "0bmKKWS04lZzoPC/KlS1kJgWN+XnvBw0yn4PPnot73E=");
/// ```
#[wasm_bindgen]
pub fn extract_key_from_bio(bio: String) -> Option<String> {
    let mut key: String = "".to_string();

    let mut bio_chars = bio.chars();

    for (i, window) in bio_chars.clone().collect::<Vec<_>>().windows(3).enumerate() {
        if window == &['🐘', '🔑', ':'] {
            let mut j = i + 3;

            if let Some(first) = bio_chars.nth(j) {
                key.push(first);

                while let Some(c) = bio_chars.next() {
                    if j < i + 46 {
                        key.push(c);
                        j += 1;
                    } else {
                        break;
                    }
                }
            } else {
                break;
            }
        }
    }

    if key.len() == 44 {
        return Some(key);
    }
    None
}

#[wasm_bindgen]
pub struct Keys {
    public: String,
    private: String,
}

#[wasm_bindgen]
impl Keys {
    #[wasm_bindgen(getter)]
    pub fn public(&self) -> String {
        self.public.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn private(&self) -> String {
        self.private.clone()
    }
}

/// for generating the pub/priv key pairs you will use for communication.
///
/// ```rust
/// let keys = encryptodon::generate_keys();
///
/// let pub_key = keys.public();
/// let priv_key = keys.private();
///
/// assert_eq!(pub_key.len(), 44);
/// assert_eq!(priv_key.len(), 44);
/// ```
#[wasm_bindgen]
pub fn generate_keys() -> Keys {
    let priv_key = x25519_dalek::StaticSecret::random_from_rng(rand::rngs::OsRng);
    let pub_key = x25519_dalek::PublicKey::from(&priv_key);

    Keys {
        public: b64.encode(pub_key.as_bytes()),
        private: b64.encode(priv_key.as_bytes()),
    }
}

/// for encrypting the communication with their pub and your priv.
///
/// ```rust
/// let their_keys = encryptodon::generate_keys();
/// let your_keys = encryptodon::generate_keys();
///
/// /* your end */
/// let status = "Pachyderm Goes Private 🔐🐘".to_string();
/// let encrypted_status = encryptodon::encrypt(status.clone(), their_keys.public(), your_keys.private()).unwrap();
///
/// /* their end */
/// let decrypted_status = encryptodon::decrypt(encrypted_status, your_keys.public(), their_keys.private()).unwrap();
///
/// assert_eq!(decrypted_status, status);
/// ```
#[wasm_bindgen]
pub fn encrypt(status: String, pub_key: String, priv_key: String) -> Result<String, String> {
    let shared_secret = string_keys_to_shared_secret(&pub_key, &priv_key)?;
    let cipher = aes::Aes256Enc::new(GenericArray::from_slice(&shared_secret));

    let status_as_bytes = status.as_bytes();
    let pad = (16 - (status_as_bytes.len() % 16)) as u8;

    let mut encrypted = vec![];

    for chunk in status_as_bytes.chunks(16) {
        let mut block = [0u8; 16];
        for (i, &v) in chunk.iter().enumerate() {
            block[i] = v;
        }

        let mut block = GenericArray::from(block);
        cipher.encrypt_block(&mut block);

        encrypted.extend(block.as_slice());
    }

    encrypted.push(pad);

    Ok(format!("🐘🔒:{}", b64.encode(encrypted)))
}

#[wasm_bindgen]
pub fn decrypt(status: String, pub_key: String, priv_key: String) -> Result<String, String> {
    let shared_secret = string_keys_to_shared_secret(&pub_key, &priv_key)?;
    let cipher = aes::Aes256Dec::new(GenericArray::from_slice(&shared_secret));

    let mut status = status;
    status.drain(..3);

    let mut status_as_bytes = match b64.decode(status) {
        Ok(s) => s,
        Err(_) => return Err("failed to decode base64 status".to_string()),
    };

    let pad = match status_as_bytes.pop() {
        Some(p) => p,
        None => return Err("decoded vec is empty".to_string()),
    };

    let mut decrypted = vec![];

    for chunk in status_as_bytes.chunks(16) {
        let mut block = [0u8; 16];
        for (i, &v) in chunk.iter().enumerate() {
            block[i] = v;
        }

        let mut block = GenericArray::from(block);
        cipher.decrypt_block(&mut block);

        decrypted.extend(block.as_slice());
    }

    if pad < 16 {
        decrypted.truncate(decrypted.len() - (pad as usize));
    }

    Ok(String::from_utf8_lossy(&decrypted).to_string())
}
