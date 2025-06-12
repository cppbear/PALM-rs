// Answer 0

#[test]
fn test_set_len_zero_length() {
    let mut b = BytesMut::with_capacity(10);
    unsafe {
        b.set_len(0);
    }
}

#[test]
fn test_set_len_full_capacity() {
    let mut b = BytesMut::with_capacity(10);
    unsafe {
        b.set_len(10);
    }
}

#[test]
fn test_set_len_edge_case() {
    let mut b = BytesMut::with_capacity(1);
    unsafe {
        b.set_len(1);
    }
}

#[test]
fn test_set_len_out_of_bounds_panics() {
    let mut b = BytesMut::with_capacity(5);
    let result = std::panic::catch_unwind(|| {
        unsafe {
            b.set_len(6);
        }
    });
    assert!(result.is_err());
}

#[test]
fn test_set_len_multiple_capacity() {
    let mut b = BytesMut::with_capacity(100);
    unsafe {
        b.set_len(100);
    }
}

#[test]
fn test_set_len_half_capacity() {
    let mut b = BytesMut::with_capacity(20);
    unsafe {
        b.set_len(10);
    }
}

