// Answer 0

#[test]
fn test_slice_new() {
    let slice: &Slice<i32> = Slice::new();
    assert_eq!(slice.len(), 0);
    assert!(slice.is_empty());
}

#[test]
fn test_slice_empty_first() {
    let slice: &Slice<i32> = Slice::new();
    assert_eq!(slice.first(), None);
}

#[test]
fn test_slice_empty_last() {
    let slice: &Slice<i32> = Slice::new();
    assert_eq!(slice.last(), None);
}

#[test]
fn test_slice_empty_split_first() {
    let slice: &Slice<i32> = Slice::new();
    assert_eq!(slice.split_first(), None);
}

#[test]
fn test_slice_empty_split_last() {
    let slice: &Slice<i32> = Slice::new();
    assert_eq!(slice.split_last(), None);
}

