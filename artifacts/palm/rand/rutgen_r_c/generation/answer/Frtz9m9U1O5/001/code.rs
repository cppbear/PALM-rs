// Answer 0

#[test]
fn test_fill_bytes_with_small_buffer() {
    let mut rng = Mcg128Xsl64 { state: 0 };
    let mut buffer: [u8; 4] = [0; 4];
    rng.fill_bytes(&mut buffer);
    assert_eq!(buffer.len(), 4);
}

#[test]
fn test_fill_bytes_with_empty_buffer() {
    let mut rng = Mcg128Xsl64 { state: 0 };
    let mut buffer: [u8; 0] = [];
    rng.fill_bytes(&mut buffer);
    assert_eq!(buffer.len(), 0);
}

#[test]
fn test_fill_bytes_with_large_buffer() {
    let mut rng = Mcg128Xsl64 { state: 0 };
    let mut buffer: [u8; 1024] = [0; 1024];
    rng.fill_bytes(&mut buffer);
    assert_eq!(buffer.len(), 1024);
}

#[should_panic]
fn test_fill_bytes_with_null_buffer() {
    let mut rng = Mcg128Xsl64 { state: 0 };
    let mut buffer: Option<&mut [u8]> = None;
    if let Some(b) = buffer {
        rng.fill_bytes(b);
    }
}

#[test]
fn test_fill_bytes_with_non_default_state() {
    let mut rng = Mcg128Xsl64 { state: 12345 };
    let mut buffer: [u8; 16] = [0; 16];
    rng.fill_bytes(&mut buffer);
    assert_eq!(buffer.len(), 16);
    assert_ne!(buffer, [0; 16]);
}

