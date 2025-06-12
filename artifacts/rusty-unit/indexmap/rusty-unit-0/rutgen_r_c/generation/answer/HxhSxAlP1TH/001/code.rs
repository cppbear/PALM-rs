// Answer 0

#[test]
fn test_split_at_with_zero_index() {
    let entries = Box::new([Bucket { hash: 0, key: 1, value: "a" }]);
    let slice = Slice::from_boxed(entries);
    
    let (first, second) = slice.split_at(0);
    
    assert_eq!(first.len(), 0);
    assert_eq!(second.len(), 1);
}

#[test]
fn test_split_at_with_full_length() {
    let entries = Box::new([
        Bucket { hash: 0, key: 1, value: "a" },
        Bucket { hash: 1, key: 2, value: "b" },
    ]);
    let slice = Slice::from_boxed(entries);
    
    let (first, second) = slice.split_at(2);
    
    assert_eq!(first.len(), 2);
    assert_eq!(second.len(), 0);
}

#[should_panic(expected = "index out of bounds")]
#[test]
fn test_split_at_with_out_of_bounds_index() {
    let entries = Box::new([Bucket { hash: 0, key: 1, value: "a" }]);
    let slice = Slice::from_boxed(entries);
    
    slice.split_at(2); // This index is out of bounds
}

#[test]
fn test_split_at_with_mid_index() {
    let entries = Box::new([
        Bucket { hash: 0, key: 1, value: "a" },
        Bucket { hash: 1, key: 2, value: "b" },
        Bucket { hash: 2, key: 3, value: "c" },
    ]);
    let slice = Slice::from_boxed(entries);
    
    let (first, second) = slice.split_at(1);
    
    assert_eq!(first.len(), 1);
    assert_eq!(second.len(), 2);
    assert_eq!(first.get_index(0).unwrap().1, &"a");
    assert_eq!(second.get_index(0).unwrap().1, &"b");
}

