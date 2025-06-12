// Answer 0

#[test]
fn test_try_unsplit_other_capacity_zero() {
    let mut bytes_mut = BytesMut::with_capacity(10); // Initial capacity > 0
    let other = BytesMut::with_capacity(0); // Other has capacity zero

    let result = bytes_mut.try_unsplit(other);
    assert!(result.is_ok()); // Should return Ok(())
}

#[test]
fn test_try_unsplit_ptrs_differ() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    // Manually set kind and data to simulate the expected conditions
    bytes_mut.kind = KIND_ARC;
    bytes_mut.len = 5;
    bytes_mut.cap = 10;
    bytes_mut.data = 0x1 as *mut u8; // Simulated data pointer

    let mut other = BytesMut::with_capacity(10);
    other.kind = KIND_ARC;
    other.len = 5;
    other.cap = 10;
    other.data = 0x2 as *mut u8; // Different data pointer

    let result = bytes_mut.try_unsplit(other);
    assert!(result.is_err()); // Should return Err(other)
    // Verify the returned Err value is still valid (is the same object)
}

#[test]
fn test_try_unsplit_different_kinds() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    bytes_mut.kind = KIND_ARC;
    bytes_mut.len = 5;
    bytes_mut.cap = 10;
    bytes_mut.data = 0x1 as *mut u8;

    let mut other = BytesMut::with_capacity(10);
    other.kind = KIND_OTHER; // Different kind
    other.len = 5;
    other.cap = 10;
    other.data = bytes_mut.data; // Same data pointer

    let result = bytes_mut.try_unsplit(other);
    assert!(result.is_err()); // Should return Err(other)
}

