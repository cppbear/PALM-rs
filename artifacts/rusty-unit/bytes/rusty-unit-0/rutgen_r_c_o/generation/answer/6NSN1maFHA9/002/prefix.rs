// Answer 0

#[test]
fn test_chunk_mut_non_empty_capacity() {
    let mut bytes_mut = BytesMut::with_capacity(128);
    unsafe {
        bytes_mut.set_len(64);
    }
    let _result = bytes_mut.chunk_mut();
}

#[test]
fn test_chunk_mut_equal_capacity() {
    let mut bytes_mut = BytesMut::with_capacity(64);
    unsafe {
        bytes_mut.set_len(64);
    }
    let _result = bytes_mut.chunk_mut();
}

#[test]
fn test_chunk_mut_scenario_below_max_capacity() {
    let mut bytes_mut = BytesMut::with_capacity(32);
    unsafe {
        bytes_mut.set_len(16);
    }
    let _result = bytes_mut.chunk_mut();
}

#[test]
fn test_chunk_mut_non_empty_and_greater_capacity() {
    let mut bytes_mut = BytesMut::with_capacity(256);
    unsafe {
        bytes_mut.set_len(128);
    }
    let _result = bytes_mut.chunk_mut();
}

