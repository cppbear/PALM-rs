// Answer 0

#[test]
fn test_remaining_mut_zero_length() {
    let mut bytes_mut = BytesMut::new();
    unsafe { bytes_mut.set_len(0) };
    bytes_mut.remaining_mut();
}

#[test]
fn test_remaining_mut_small_length() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    unsafe { bytes_mut.set_len(5) };
    bytes_mut.remaining_mut();
}

#[test]
fn test_remaining_mut_large_length() {
    let mut bytes_mut = BytesMut::with_capacity(1024);
    unsafe { bytes_mut.set_len(512) };
    bytes_mut.remaining_mut();
}

#[test]
fn test_remaining_mut_half_capacity() {
    let mut bytes_mut = BytesMut::with_capacity(usize::MAX);
    unsafe { bytes_mut.set_len(usize::MAX / 2) };
    bytes_mut.remaining_mut();
}

#[test]
fn test_remaining_mut_max_length() {
    let mut bytes_mut = BytesMut::new();
    unsafe { bytes_mut.set_len(usize::MAX) };
    bytes_mut.remaining_mut();
}

