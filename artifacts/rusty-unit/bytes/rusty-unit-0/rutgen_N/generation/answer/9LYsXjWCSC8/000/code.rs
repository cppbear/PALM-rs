// Answer 0

#[test]
fn test_as_slice_valid() {
    struct ByteArray {
        ptr: *const u8,
        len: usize,
    }

    let data = [1, 2, 3, 4, 5];
    let byte_array = ByteArray {
        ptr: data.as_ptr(),
        len: data.len(),
    };

    let slice = unsafe { std::slice::from_raw_parts(byte_array.ptr, byte_array.len) };
    assert_eq!(slice, [1, 2, 3, 4, 5]);
}

#[test]
fn test_as_slice_empty() {
    struct ByteArray {
        ptr: *const u8,
        len: usize,
    }

    let byte_array = ByteArray {
        ptr: std::ptr::null(),
        len: 0,
    };

    let slice = unsafe { std::slice::from_raw_parts(byte_array.ptr, byte_array.len) };
    assert_eq!(slice, []);
}

