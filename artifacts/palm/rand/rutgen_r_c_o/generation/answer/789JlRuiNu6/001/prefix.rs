// Answer 0

#[test]
fn test_set_block_pos_zero() {
    let key: [u8; 32] = [0; 32];
    let nonce: [u8; 8] = [0; 8];
    let mut state = ChaCha::new(&key, &nonce);
    state.set_block_pos(0);
}

#[test]
fn test_set_block_pos_max() {
    let key: [u8; 32] = [0; 32];
    let nonce: [u8; 8] = [0; 8];
    let mut state = ChaCha::new(&key, &nonce);
    state.set_block_pos(u64::max_value());
}

#[test]
fn test_set_block_pos_mid() {
    let key: [u8; 32] = [0; 32];
    let nonce: [u8; 8] = [0; 8];
    let mut state = ChaCha::new(&key, &nonce);
    state.set_block_pos(1_000_000);
}

#[test]
fn test_set_block_pos_edge() {
    let key: [u8; 32] = [0; 32];
    let nonce: [u8; 8] = [0; 8];
    let mut state = ChaCha::new(&key, &nonce);
    state.set_block_pos(2);
}

