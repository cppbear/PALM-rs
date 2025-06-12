// Answer 0

#[test]
fn test_fill_with_empty_slice() {
    let mut data: [u8; 0] = [];
    fastrand::fill(&mut data);
    assert!(data.is_empty());
}

#[test]
fn test_fill_with_small_slice() {
    let mut data = [0u8; 4];
    fastrand::fill(&mut data);
    assert_ne!(data, [0, 0, 0, 0]);
}

#[test]
fn test_fill_with_large_slice() {
    let mut data = [0u8; 1024];
    fastrand::fill(&mut data);
    assert_ne!(data, [0; 1024]);
}

#[test]
#[should_panic]
fn test_fill_with_null_slice() {
    let slice: &mut [u8] = std::ptr::null_mut();
    fastrand::fill(slice);
}

