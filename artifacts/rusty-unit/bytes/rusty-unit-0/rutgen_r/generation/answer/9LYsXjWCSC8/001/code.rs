// Answer 0

#[test]
fn test_as_slice_valid_pointer() {
    struct Bytes {
        ptr: *const u8,
        len: usize,
    }

    let data = [1u8, 2, 3, 4];
    let slice = &data as *const _ as *const u8;
    let bytes = Bytes { ptr: slice, len: data.len() };

    let result = unsafe { std::slice::from_raw_parts(bytes.ptr, bytes.len) };
    assert_eq!(result, &data);
}

#[test]
#[should_panic]
fn test_as_slice_zero_length() {
    struct Bytes {
        ptr: *const u8,
        len: usize,
    }

    let bytes = Bytes { ptr: std::ptr::null(), len: 0 };

    let _result = unsafe { std::slice::from_raw_parts(bytes.ptr, bytes.len) }; // should panic
}

#[test]
#[should_panic]
fn test_as_slice_invalid_pointer() {
    struct Bytes {
        ptr: *const u8,
        len: usize,
    }

    let bytes = Bytes { ptr: std::ptr::null(), len: 10 };

    let _result = unsafe { std::slice::from_raw_parts(bytes.ptr, bytes.len) }; // should panic
}

#[test]
fn test_as_slice_non_null_pointer() {
    struct Bytes {
        ptr: *const u8,
        len: usize,
    }

    let data = [5u8, 6, 7, 8];
    let slice = &data as *const _ as *const u8;
    let bytes = Bytes { ptr: slice, len: 4 };

    let result = unsafe { std::slice::from_raw_parts(bytes.ptr, bytes.len) };
    assert_eq!(result, &data);
}

