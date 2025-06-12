// Answer 0

#[test]
fn test_advance_within_bounds() {
    let mut bytes = Bytes::from_static(b"hello");
    let initial_len = bytes.len();
    
    bytes.advance(3);
    
    assert_eq!(bytes.len(), initial_len - 3);
    assert_eq!(bytes.chunk(), b"lo");
}

#[test]
#[should_panic(expected = "cannot advance past `remaining`: 5 <= 5")]
fn test_advance_panic_at_len() {
    let mut bytes = Bytes::from_static(b"hello");
    
    bytes.advance(5);
}

#[test]
#[should_panic(expected = "cannot advance past `remaining`: 6 <= 5")]
fn test_advance_panic_over_len() {
    let mut bytes = Bytes::from_static(b"hello");
    
    bytes.advance(6);
}

#[test]
fn test_advance_no_change_when_zero() {
    let mut bytes = Bytes::from_static(b"hello");
    let initial_len = bytes.len();
    
    bytes.advance(0);
    
    assert_eq!(bytes.len(), initial_len);
    assert_eq!(bytes.chunk(), b"hello");
}

