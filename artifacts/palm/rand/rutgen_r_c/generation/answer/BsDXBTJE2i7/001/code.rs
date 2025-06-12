// Answer 0

#[test]
fn test_fill_bytes_empty_slice() {
    let mut rng = StepRng { v: 1, a: 1 };
    let mut buffer: [u8; 0] = [];
    rng.fill_bytes(&mut buffer);
    assert_eq!(buffer, []);
}

#[test]
fn test_fill_bytes_small_slice() {
    let mut rng = StepRng { v: 1, a: 2 };
    let mut buffer = [0u8; 4];
    rng.fill_bytes(&mut buffer);
    assert_eq!(buffer.len(), 4);
}

#[test]
fn test_fill_bytes_large_slice() {
    let mut rng = StepRng { v: 1, a: 3 };
    let mut buffer = [0u8; 1024];
    rng.fill_bytes(&mut buffer);
    assert_eq!(buffer.len(), 1024);
}

#[should_panic]
fn test_fill_bytes_null_slice() {
    let mut rng = StepRng { v: 1, a: 4 };
    rng.fill_bytes(std::ptr::null_mut());
}

