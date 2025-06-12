// Answer 0

#[test]
fn test_is_empty_on_empty_slice() {
    struct Dummy;

    let empty_slice = Slice::<Dummy> {
        entries: [],
    };
    
    assert!(empty_slice.is_empty());
}

#[test]
fn test_is_empty_on_non_empty_slice() {
    struct Dummy;

    let non_empty_slice = Slice::<Dummy> {
        entries: [Bucket { hash: 0, key: Dummy, value: () }],
    };
    
    assert!(!non_empty_slice.is_empty());
}

