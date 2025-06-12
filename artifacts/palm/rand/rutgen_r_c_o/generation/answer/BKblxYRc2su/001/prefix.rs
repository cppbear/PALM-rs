// Answer 0

#[test]
fn test_set_nonce_zero() {
    let key: [u8; 32] = [0; 32];
    let nonce: [u8; 12] = [0; 12];
    let mut chacha = ChaCha::new(&key, &nonce);
    chacha.set_nonce(0);
}

#[test]
fn test_set_nonce_one() {
    let key: [u8; 32] = [0; 32];
    let nonce: [u8; 12] = [0; 12];
    let mut chacha = ChaCha::new(&key, &nonce);
    chacha.set_nonce(1);
}

#[test]
fn test_set_nonce_two() {
    let key: [u8; 32] = [0; 32];
    let nonce: [u8; 12] = [0; 12];
    let mut chacha = ChaCha::new(&key, &nonce);
    chacha.set_nonce(2);
}

#[test]
fn test_set_nonce_three() {
    let key: [u8; 32] = [0; 32];
    let nonce: [u8; 12] = [0; 12];
    let mut chacha = ChaCha::new(&key, &nonce);
    chacha.set_nonce(3);
}

#[test]
fn test_set_nonce_four() {
    let key: [u8; 32] = [0; 32];
    let nonce: [u8; 12] = [0; 12];
    let mut chacha = ChaCha::new(&key, &nonce);
    chacha.set_nonce(4);
}

#[test]
fn test_set_nonce_max() {
    let key: [u8; 32] = [0; 32];
    let nonce: [u8; 12] = [0; 12];
    let mut chacha = ChaCha::new(&key, &nonce);
    chacha.set_nonce(u64::MAX);
}

