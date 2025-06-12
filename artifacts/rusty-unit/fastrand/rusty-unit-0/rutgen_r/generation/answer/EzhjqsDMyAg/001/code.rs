// Answer 0

#[test]
fn test_fill_with_empty_slice() {
    let mut buf: [u8; 0] = [];
    fill(&mut buf);
    assert_eq!(buf.len(), 0);
}

#[test]
fn test_fill_with_small_slice() {
    let mut buf: [u8; 4] = [0; 4];
    fill(&mut buf);
    assert_ne!(buf, [0; 4]);
}

#[test]
fn test_fill_with_large_slice() {
    let mut buf: [u8; 32] = [0; 32];
    fill(&mut buf);
    assert_ne!(buf, [0; 32]);
}

#[test]
fn test_fill_with_boundary_slice() {
    let mut buf: [u8; 1] = [0; 1];
    fill(&mut buf);
    assert_ne!(buf, [0; 1]);
}

#[test]
#[should_panic]
fn test_fill_with_null_slice() {
    let slice: *mut u8 = std::ptr::null_mut();
    unsafe {
        fill(std::slice::from_raw_parts_mut(slice, 0));
    }
}

