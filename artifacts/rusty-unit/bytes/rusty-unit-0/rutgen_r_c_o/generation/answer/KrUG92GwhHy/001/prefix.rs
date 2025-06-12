// Answer 0

#[test]
fn test_write_str_empty() {
    let mut bytes_mut = BytesMut {
        ptr: NonNull::new(unsafe { core::ptr::null_mut() }).unwrap(),
        len: 0,
        cap: 0,
        data: std::ptr::null_mut(),
    };
    bytes_mut.len = 0; // self.remaining_mut() == 0
    bytes_mut.cap = 0; // self.remaining_mut() == 0
    let result = bytes_mut.write_str("");
}

#[test]
fn test_write_str_exact_fit() {
    let mut bytes_mut = BytesMut {
        ptr: NonNull::new(unsafe { core::ptr::null_mut() }).unwrap(),
        len: 4,
        cap: 4,
        data: std::ptr::null_mut(),
    };
    bytes_mut.len = 4; // self.remaining_mut() == 4
    bytes_mut.cap = 4; // self.remaining_mut() == 4
    let result = bytes_mut.write_str("test");
}

#[test]
fn test_write_str_spaces() {
    let mut bytes_mut = BytesMut {
        ptr: NonNull::new(unsafe { core::ptr::null_mut() }).unwrap(),
        len: 5,
        cap: 5,
        data: std::ptr::null_mut(),
    };
    bytes_mut.len = 5; // self.remaining_mut() == 5
    bytes_mut.cap = 5; // self.remaining_mut() == 5
    let result = bytes_mut.write_str("hello");
}

