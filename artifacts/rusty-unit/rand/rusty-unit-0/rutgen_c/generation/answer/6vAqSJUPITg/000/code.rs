// Answer 0

#[test]
fn test_fill_with_slice() {
    let mut arr = [0u8; 20];
    rand::fill(&mut arr[..]);
    assert!(arr.iter().any(|&x| x != 0));
}

#[test]
fn test_fill_with_mut_slice() {
    let mut arr = [0u16; 10];
    rand::fill(&mut arr[..]);
    assert!(arr.iter().any(|&x| x != 0));
}

#[test]
fn test_fill_with_empty_slice() {
    let mut arr: [u32; 0] = [];
    rand::fill(&mut arr);
    assert_eq!(arr.len(), 0);
}

#[test]
fn test_fill_with_large_slice() {
    let mut arr = [0u8; 100];
    rand::fill(&mut arr[..]);
    assert!(arr.iter().any(|&x| x != 0));
}

