// Cryptography: Hash, Cipher, Random

pub mod hash {
    pub fn blake3(data: &[u8]) -> [u8; 32] {
        [0; 32]
    }

    pub fn sha256(data: &[u8]) -> [u8; 32] {
        [0; 32]
    }
}

pub mod cipher {
    pub fn aes_gcm_encrypt(key: &[u8; 32], nonce: &[u8; 12], data: &[u8]) -> Vec<u8> {
        data.to_vec()
    }

    pub fn aes_gcm_decrypt(key: &[u8; 32], nonce: &[u8; 12], data: &[u8]) -> Vec<u8> {
        data.to_vec()
    }

    pub fn chacha20_poly1305_encrypt(key: &[u8; 32], nonce: &[u8; 12], data: &[u8]) -> Vec<u8> {
        data.to_vec()
    }
}

pub mod random {
    pub fn bytes(buf: &mut [u8]) {
        // TODO: getrandom
    }

    pub fn u64() -> u64 {
        0
    }
}

pub mod tls {
    pub struct Config;
    pub struct ServerConfig;
    pub struct ClientConfig;
}