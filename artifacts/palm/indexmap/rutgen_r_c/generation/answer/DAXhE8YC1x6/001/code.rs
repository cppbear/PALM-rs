// Answer 0


#[test]
fn test_iter_empty_slice() {
    let slice: &Slice<i32> = Slice::new();
    let mut iter = slice.iter();
    assert!(iter.iter.len() == 0);
}

#[test]
fn test_iter_non_empty_slice() {
    // Create a non-empty Slice explicitly.
    let entries = [Bucket { hash: 0, key: 1, value: "a" }, Bucket { hash: 0, key: 2, value: "b" }];
    let slice = Slice { entries };
    let mut iter = slice.iter();
    
    assert_eq!(iter.iter.len(), 2);
    assert_eq!(iter.as_slice().entries.len(), 2);
}

#[test]
fn test_iter_single_element() {
    // Create a Slice with a single Bucket.
    let entries = [Bucket { hash: 0, key: 1, value: "a" }];
    let slice = Slice { entries };
    let mut iter = slice.iter();
    
    assert_eq!(iter.iter.len(), 1);
    assert_eq!(iter.as_slice().entries.len(), 1);
}



