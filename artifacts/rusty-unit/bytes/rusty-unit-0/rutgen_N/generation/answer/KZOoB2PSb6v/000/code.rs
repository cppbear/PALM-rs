// Answer 0

#[test]
fn test_freeze_with_vec() {
    use bytes::{BytesMut, Bytes};

    // Initialize a BytesMut with capacity
    let mut b = BytesMut::with_capacity(64);
    b.put(&b"hello world"[..]);
    
    // Call freeze to convert to Bytes
    let b1: Bytes = b.freeze();
    
    // Clone Bytes
    let b2 = b1.clone();
    
    // Assert that both Bytes instances equal the original slice
    assert_eq!(&b1[..], b"hello world");
    assert_eq!(&b2[..], b"hello world");
}

#[test]
fn test_freeze_with_arbitrary_data() {
    use bytes::{BytesMut, Bytes};

    // Initialize a BytesMut with capacity
    let mut b = BytesMut::with_capacity(64);
    b.put(&b"arbitrary data"[..]);
    
    // Call freeze to convert to Bytes
    let b1: Bytes = b.freeze();
    
    // Clone Bytes
    let b2 = b1.clone();
    
    // Assert that both Bytes instances equal the original slice
    assert_eq!(&b1[..], b"arbitrary data");
    assert_eq!(&b2[..], b"arbitrary data");
}

#[should_panic]
fn test_freeze_panic_on_mutation() {
    use bytes::{BytesMut, Bytes};

    // Initialize a BytesMut with capacity
    let mut b = BytesMut::with_capacity(64);
    b.put(&b"mutable data"[..]);
    
    // Freeze the BytesMut
    let _frozen: Bytes = b.freeze();

    // Attempting to mutate frozen data should panic
    b.put(&b"additional data"[..]);
}

