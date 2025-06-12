// Answer 0

#[test]
fn test_get_nonce_with_default_value() {
    let key: [u8; 32] = [0; 32];
    let nonce: &[u8] = &[0; 12];
    let mut chacha = ChaCha::new(&key, nonce);
    let nonce_value = chacha.get_nonce();
    assert_eq!(nonce_value, 0); // Since we initialized it with zeroes
}

#[test]
fn test_get_nonce_with_non_zero_nonce() {
    let key: [u8; 32] = [1; 32];
    let nonce: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    let mut chacha = ChaCha::new(&key, nonce);
    let nonce_value = chacha.get_nonce();
    assert_ne!(nonce_value, 0); // Assert that the nonce is not zero
}

