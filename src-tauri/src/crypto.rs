use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use rand::RngCore;
use sha2::{Sha256, Digest};
use std::io::{Read, Write};

pub struct FileEncryption {
    cipher: Aes256Gcm,
    nonce: Vec<u8>,
}

impl FileEncryption {
    pub fn new(password: &str) -> Self {
        // 使用密码生成密钥
        let mut hasher = Sha256::new();
        hasher.update(password.as_bytes());
        let key_bytes = hasher.finalize();
        let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
        
        let cipher = Aes256Gcm::new(key);
        
        // 生成随机nonce
        let mut nonce = vec![0u8; 12];
        OsRng.fill_bytes(&mut nonce);
        
        Self { cipher, nonce }
    }
    
    pub fn from_key_and_nonce(password: &str, nonce: Vec<u8>) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(password.as_bytes());
        let key_bytes = hasher.finalize();
        let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
        
        let cipher = Aes256Gcm::new(key);
        
        Self { cipher, nonce }
    }
    
    pub fn encrypt_chunk(&self, plaintext: &[u8]) -> Result<Vec<u8>, String> {
        let nonce = Nonce::from_slice(&self.nonce);
        self.cipher
            .encrypt(nonce, plaintext)
            .map_err(|e| format!("Encryption failed: {}", e))
    }
    
    pub fn decrypt_chunk(&self, ciphertext: &[u8]) -> Result<Vec<u8>, String> {
        let nonce = Nonce::from_slice(&self.nonce);
        self.cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| format!("Decryption failed: {}", e))
    }
    
    pub fn get_nonce(&self) -> &[u8] {
        &self.nonce
    }
}

// 文件加密结构
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EncryptedFileHeader {
    pub original_size: u64,
    pub nonce: Vec<u8>,
    pub chunk_size: usize,
}

pub fn generate_secure_code() -> String {
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};
    
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect::<String>()
        .to_uppercase()
} 