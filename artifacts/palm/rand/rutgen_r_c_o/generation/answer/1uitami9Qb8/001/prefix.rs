// Answer 0

#[test]
fn test_get_nonce_zero_nonce() {
    let key = [0u8; 32];
    let nonce = &[0u8; 8];
    let mut chacha = ChaCha::new(&key, nonce);
    let _ = chacha.get_nonce();
}

#[test]
fn test_get_nonce_small_nonce() {
    let key = [1u8; 32];
    let nonce = &[1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8];
    let mut chacha = ChaCha::new(&key, nonce);
    let _ = chacha.get_nonce();
}

#[test]
fn test_get_nonce_large_nonce() {
    let key = [2u8; 32];
    let nonce = &[255u8; 8];
    let mut chacha = ChaCha::new(&key, nonce);
    let _ = chacha.get_nonce();
}

#[test]
fn test_get_nonce_boundary_value() {
    let key = [3u8; 32];
    let nonce = &[0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8];
    let mut chacha = ChaCha::new(&key, nonce);
    let _ = chacha.get_nonce();
}

#[test]
fn test_get_nonce_non_zero_boundary_value() {
    let key = [4u8; 32];
    let nonce = &[0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 255u8];
    let mut chacha = ChaCha::new(&key, nonce);
    let _ = chacha.get_nonce();
}

