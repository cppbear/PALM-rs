// Answer 0

#[test]
fn test_refill4_with_zero_drounds() {
    let key: [u8; 32] = [0; 32];
    let nonce: [u8; 12] = [0; 12];
    let mut chacha = ChaCha::new(&key, &nonce);
    let mut output: [u32; BUFSZ] = [0; BUFSZ];
    chacha.refill4(0, &mut output);
}

#[test]
fn test_refill4_with_min_drounds() {
    let key: [u8; 32] = [1; 32];
    let nonce: [u8; 12] = [1; 12];
    let mut chacha = ChaCha::new(&key, &nonce);
    let mut output: [u32; BUFSZ] = [0; BUFSZ];
    chacha.refill4(1, &mut output);
}

#[test]
fn test_refill4_with_max_drounds() {
    let key: [u8; 32] = [2; 32];
    let nonce: [u8; 12] = [2; 12];
    let mut chacha = ChaCha::new(&key, &nonce);
    let mut output: [u32; BUFSZ] = [0; BUFSZ];
    chacha.refill4(64, &mut output);
}

#[test]
fn test_refill4_with_varied_key_and_nonce() {
    let key: [u8; 32] = [3; 32];
    let nonce: [u8; 24] = [3; 24];
    let mut chacha = ChaCha::new(&key, &nonce);
    let mut output: [u32; BUFSZ] = [0; BUFSZ];
    chacha.refill4(32, &mut output);
}

#[test]
fn test_refill4_with_full_output_buffer() {
    let key: [u8; 32] = [4; 32];
    let nonce: [u8; 12] = [4; 12];
    let mut chacha = ChaCha::new(&key, &nonce);
    let mut output: [u32; BUFSZ] = [u32::MAX; BUFSZ];
    chacha.refill4(16, &mut output);
}

