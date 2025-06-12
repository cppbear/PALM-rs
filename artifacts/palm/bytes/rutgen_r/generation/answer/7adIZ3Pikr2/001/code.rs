// Answer 0

#[test]
fn test_remaining_mut_zero_length() {
    let buf = Vec::<u8>::with_capacity(0);
    assert_eq!(buf.remaining_mut(), core::isize::MAX as usize);
}

#[test]
fn test_remaining_mut_small_length() {
    let buf = Vec::<u8>::with_capacity(10);
    assert_eq!(buf.remaining_mut(), core::isize::MAX as usize - 10);
}

#[test]
fn test_remaining_mut_large_length() {
    let buf = Vec::<u8>::with_capacity(core::isize::MAX as usize - 1);
    assert_eq!(buf.remaining_mut(), 1);
}

#[test]
fn test_remaining_mut_max_length() {
    let buf = Vec::<u8>::with_capacity(core::isize::MAX as usize);
    assert_eq!(buf.remaining_mut(), 0);
}

