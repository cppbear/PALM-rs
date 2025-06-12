// Answer 0

#[test]
fn test_chunk_empty() {
    let bytes_mut = BytesMut::new();
    let result = bytes_mut.chunk();
}

#[test]
fn test_chunk_uninitialized() {
    let len = 0;
    let cap = 1024;
    let mut bytes_mut = BytesMut::with_capacity(cap);
    unsafe { bytes_mut.set_len(len) };
    let result = bytes_mut.chunk();
}

#[test]
fn test_chunk_full_capacity() {
    let cap = 16;
    let mut bytes_mut = BytesMut::with_capacity(cap);
    unsafe {
        bytes_mut.set_len(cap);
        ptr::write_bytes(bytes_mut.ptr.as_ptr(), 1, cap);
    }
    let result = bytes_mut.chunk();
}

#[test]
fn test_chunk_partial_capacity() {
    let cap = 32;
    let len = 20;
    let mut bytes_mut = BytesMut::with_capacity(cap);
    unsafe {
        bytes_mut.set_len(len);
        ptr::write_bytes(bytes_mut.ptr.as_ptr(), 2, len);
    }
    let result = bytes_mut.chunk();
}

#[test]
fn test_chunk_exact_capacity() {
    let cap = 1 << 17;
    let mut bytes_mut = BytesMut::with_capacity(cap);
    unsafe {
        bytes_mut.set_len(cap);
        ptr::write_bytes(bytes_mut.ptr.as_ptr(), 3, cap);
    }
    let result = bytes_mut.chunk();
}

