// Answer 0

#[test]
fn test_fill_array_int() {
    let mut arr = [0i8; 20];
    rand::fill(&mut arr[..]);
    assert!(arr.iter().all(|&x| x != 0));
}

#[test]
fn test_fill_empty_slice() {
    let mut arr: [u8; 0] = [];
    rand::fill(&mut arr[..]);
    assert!(arr.is_empty());
}

#[should_panic]
fn test_fill_invalid_input() {
    let mut invalid: &mut [u8] = std::slice::from_raw_parts_mut(0 as *mut u8, 10);
    rand::fill(&mut invalid);
}

