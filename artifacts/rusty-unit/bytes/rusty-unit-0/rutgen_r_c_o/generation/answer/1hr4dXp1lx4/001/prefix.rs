// Answer 0

#[test]
fn test_shallow_clone_arc() {
    let mut bytes_mut = BytesMut::new();
    unsafe {
        bytes_mut.promote_to_shared(1); // This sets up the BytesMut to be KIND_ARC
        let cloned_bytes_mut = bytes_mut.shallow_clone();
    }
}

#[test]
fn test_shallow_clone_arc_with_non_zero_len() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    unsafe {
        bytes_mut.promote_to_shared(1); // This sets up the BytesMut to be KIND_ARC
        bytes_mut.resize(5, 0); // Set length greater than zero
        let cloned_bytes_mut = bytes_mut.shallow_clone();
    }
}

#[test]
fn test_shallow_clone_arc_with_max_capacity() {
    let mut bytes_mut = BytesMut::with_capacity(usize::MAX);
    unsafe {
        bytes_mut.promote_to_shared(1); // This sets up the BytesMut to be KIND_ARC
        let cloned_bytes_mut = bytes_mut.shallow_clone();
    }
}

#[test]
#[should_panic]
fn test_shallow_clone_arc_with_invalid_ref_count() {
    let mut bytes_mut = BytesMut::new();
    unsafe {
        bytes_mut.promote_to_shared(0); // Invalid ref count, should panic
        let cloned_bytes_mut = bytes_mut.shallow_clone();
    }
}

#[test]
fn test_shallow_clone_arc_multiple_clones() {
    let mut bytes_mut = BytesMut::with_capacity(20);
    unsafe {
        bytes_mut.promote_to_shared(1); // This sets up the BytesMut to be KIND_ARC
        let cloned_bytes_mut_1 = bytes_mut.shallow_clone();
        let cloned_bytes_mut_2 = bytes_mut.shallow_clone();
    }
}

