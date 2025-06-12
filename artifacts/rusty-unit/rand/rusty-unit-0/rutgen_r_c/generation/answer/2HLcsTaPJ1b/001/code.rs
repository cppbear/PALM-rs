// Answer 0

#[test]
fn test_fill_bytes_empty_buffer() {
    let mut rng = Lcg128Xsl64 {
        state: 0,
        increment: 1,
    };
    let mut dest: [u8; 0] = [];
    rng.fill_bytes(&mut dest);
}

#[test]
fn test_fill_bytes_small_buffer() {
    let mut rng = Lcg128Xsl64 {
        state: 0,
        increment: 1,
    };
    let mut dest = [0u8; 1];
    rng.fill_bytes(&mut dest);
    assert!(dest[0] != 0); // Check if it was filled with a non-zero value
}

#[test]
fn test_fill_bytes_large_buffer() {
    let mut rng = Lcg128Xsl64 {
        state: 0,
        increment: 1,
    };
    let mut dest = [0u8; 256];
    rng.fill_bytes(&mut dest);
    for &byte in &dest {
        assert!(byte != 0); // Check if each byte was filled with a non-zero value
    }
}

#[should_panic]
#[test]
fn test_fill_bytes_null_buffer() {
    let mut rng = Lcg128Xsl64 {
        state: 0,
        increment: 1,
    };
    rng.fill_bytes(std::ptr::null_mut()); // This should panic
}

