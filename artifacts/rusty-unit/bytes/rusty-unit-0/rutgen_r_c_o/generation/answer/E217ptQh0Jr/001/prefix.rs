// Answer 0

#[test]
fn test_free_boxed_slice_valid_case() {
    let buf_size = 64;
    let buf = unsafe { Box::into_raw(Box::new_zeroed![u8; buf_size]) };
    let offset = unsafe { buf.add(10) };
    let len = 32;

    unsafe {
        free_boxed_slice(buf, offset, len);
    }
}

#[test]
fn test_free_boxed_slice_exact_size() {
    let buf_size = 128;
    let buf = unsafe { Box::into_raw(Box::new_zeroed![u8; buf_size]) };
    let offset = unsafe { buf.add(64) };
    let len = 64;

    unsafe {
        free_boxed_slice(buf, offset, len);
    }
}

#[test]
fn test_free_boxed_slice_min_len() {
    let buf_size = 16;
    let buf = unsafe { Box::into_raw(Box::new_zeroed![u8; buf_size]) };
    let offset = unsafe { buf.add(1) };
    let len = 1;

    unsafe {
        free_boxed_slice(buf, offset, len);
    }
}

#[test]
fn test_free_boxed_slice_non_minimal_len() {
    let buf_size = 256;
    let buf = unsafe { Box::into_raw(Box::new_zeroed![u8; buf_size]) };
    let offset = unsafe { buf.add(50) };
    let len = 200;

    unsafe {
        free_boxed_slice(buf, offset, len);
    }
}

#[test]
fn test_free_boxed_slice_boundaries() {
    let buf_size = 32;
    let buf = unsafe { Box::into_raw(Box::new_zeroed![u8; buf_size]) };
    let offset = unsafe { buf.add(31) };
    let len = 1;

    unsafe {
        free_boxed_slice(buf, offset, len);
    }
}

