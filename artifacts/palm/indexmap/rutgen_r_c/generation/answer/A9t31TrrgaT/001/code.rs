// Answer 0

#[test]
fn test_slice_new() {
    let slice: &Slice<u32> = Slice::new();
    assert!(slice.entries.is_empty());
}

#[test]
fn test_slice_new_len() {
    let slice: &Slice<u32> = Slice::new();
    assert_eq!(slice.len(), 0);
}

#[test]
fn test_slice_new_is_empty() {
    let slice: &Slice<u32> = Slice::new();
    assert!(slice.is_empty());
}

#[test]
fn test_slice_new_first() {
    let slice: &Slice<u32> = Slice::new();
    assert!(slice.first().is_none());
}

#[test]
fn test_slice_new_last() {
    let slice: &Slice<u32> = Slice::new();
    assert!(slice.last().is_none());
}

#[test]
fn test_slice_new_split_first() {
    let slice: &Slice<u32> = Slice::new();
    assert!(slice.split_first().is_none());
}

#[test]
fn test_slice_new_split_last() {
    let slice: &Slice<u32> = Slice::new();
    assert!(slice.split_last().is_none());
}

