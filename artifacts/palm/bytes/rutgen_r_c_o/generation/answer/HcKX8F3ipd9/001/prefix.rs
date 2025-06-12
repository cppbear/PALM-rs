// Answer 0

#[test]
fn test_as_slice_non_empty() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    unsafe {
        bytes_mut.set_len(5);
        let slice = bytes_mut.as_slice();
    }
}

#[test]
fn test_as_slice_empty() {
    let bytes_mut = BytesMut::new();
    let slice = bytes_mut.as_slice();
}

#[test]
fn test_as_slice_full_capacity() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    unsafe {
        bytes_mut.set_len(10);
        let slice = bytes_mut.as_slice();
    }
}

#[test]
#[should_panic]
fn test_as_slice_out_of_bounds() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    unsafe {
        bytes_mut.set_len(15); // Exceeds allocated capacity
        let slice = bytes_mut.as_slice();
    }
}

#[test]
fn test_as_slice_max_length() {
    let mut bytes_mut = BytesMut::with_capacity(usize::MAX);
    unsafe {
        bytes_mut.set_len(usize::MAX);
        let slice = bytes_mut.as_slice();
    }
}

