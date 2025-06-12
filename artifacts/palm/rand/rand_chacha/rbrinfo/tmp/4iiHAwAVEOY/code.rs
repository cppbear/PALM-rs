pub fn new(key: &[u8; 32], nonce: &[u8]) -> Self {
        init_chacha(key, nonce)
    }