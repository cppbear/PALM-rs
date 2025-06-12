// Answer 0

#[test]
fn test_chunk_mut_capacity_equal_len() {
    let mut bytes_mut = BytesMut::with_capacity(64);
    unsafe { bytes_mut.set_len(64) };
    let _ = bytes_mut.chunk_mut();
}

#[test]
fn test_chunk_mut_capacity_equal_len_zero() {
    let mut bytes_mut = BytesMut::new();
    unsafe { bytes_mut.set_len(0) };
    let _ = bytes_mut.chunk_mut();
}

#[test]
fn test_chunk_mut_capacity_equal_len_mid() {
    let mut bytes_mut = BytesMut::with_capacity(32);
    unsafe { bytes_mut.set_len(32) };
    let _ = bytes_mut.chunk_mut();
}

#[test]
fn test_chunk_mut_capacity_equal_len_half() {
    let mut bytes_mut = BytesMut::with_capacity(64);
    unsafe { bytes_mut.set_len(32) };
    let _ = bytes_mut.chunk_mut();
}

#[test]
fn test_chunk_mut_capacity_full_and_reserve() {
    let mut bytes_mut = BytesMut::with_capacity(64);
    unsafe { bytes_mut.set_len(64) };
    let _ = bytes_mut.chunk_mut(); 
    let _ = bytes_mut.chunk_mut(); // should reserve again on second call
}

