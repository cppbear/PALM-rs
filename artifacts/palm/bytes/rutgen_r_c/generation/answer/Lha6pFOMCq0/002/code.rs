// Answer 0

#[test]
fn test_try_unsplit_success_empty() {
    let mut self_bytes_mut = BytesMut::with_capacity(10);
    let other_bytes_mut = BytesMut::new(); // other.capacity() == 0

    let result = self_bytes_mut.try_unsplit(other_bytes_mut);
    
    assert_eq!(result, Ok(()));
}

#[test]
fn test_try_unsplit_success_contiguous() {
    let mut self_bytes_mut = BytesMut::with_capacity(20);
    unsafe { self_bytes_mut.set_len(10); } // Set length for self
    let mut other_bytes_mut = BytesMut::with_capacity(20);
    unsafe { other_bytes_mut.set_len(5); } // Set length for other
    unsafe { self_bytes_mut.promote_to_shared(1); } // Set to KIND_ARC
    unsafe { other_bytes_mut.promote_to_shared(1); } // Set to KIND_ARC

    // Ensure they point to the same memory before unsplit (simulated)
    self_bytes_mut.data = other_bytes_mut.data;

    let result = self_bytes_mut.try_unsplit(other_bytes_mut);
    
    assert_eq!(result, Ok(()));
    assert_eq!(self_bytes_mut.len(), 15);
    assert_eq!(self_bytes_mut.capacity(), 20);
}

#[test]
fn test_try_unsplit_failure_different_ptr() {
    let mut self_bytes_mut = BytesMut::with_capacity(20);
    let other_bytes_mut = BytesMut::with_capacity(20); // other.capacity() == 20

    let result = self_bytes_mut.try_unsplit(other_bytes_mut);
    
    assert!(result.is_err());
}

#[test]
fn test_try_unsplit_failure_different_kind() {
    let mut self_bytes_mut = BytesMut::with_capacity(20);
    let mut other_bytes_mut = BytesMut::with_capacity(20);

    // Not promoting to KIND_ARC
    unsafe { self_bytes_mut.set_len(10); }
    unsafe { other_bytes_mut.set_len(10); }

    let result = self_bytes_mut.try_unsplit(other_bytes_mut);
    
    assert!(result.is_err());
}

#[test]
fn test_try_unsplit_with_capacity_zero() {
    let mut self_bytes_mut = BytesMut::with_capacity(20);
    unsafe { self_bytes_mut.set_len(15); } // Set length for self
    let other_bytes_mut = BytesMut::new(); // other.capacity() == 0

    let result = self_bytes_mut.try_unsplit(other_bytes_mut);
    
    assert_eq!(result, Ok(()));
}

