// Answer 0

#[test]
fn test_split_at_valid_index() {
    let slice = Box::new(Slice { entries: [
        Bucket { hash: 0, key: 1, value: "a" },
        Bucket { hash: 0, key: 2, value: "b" },
        Bucket { hash: 0, key: 3, value: "c" },
    ]});

    let (first, second) = slice.as_ref().split_at(2);
    assert_eq!(first.len(), 2);
    assert_eq!(second.len(), 1);
    assert_eq!(first.get_index(0), Some(&1));
    assert_eq!(first.get_index(1), Some(&2));
    assert_eq!(second.get_index(0), Some(&3));
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_split_at_panic_on_out_of_bounds() {
    let slice = Box::new(Slice { entries: [
        Bucket { hash: 0, key: 1, value: "a" },
        Bucket { hash: 0, key: 2, value: "b" },
    ]});

    // This should panic because the index is greater than the length of the slice.
    let _ = slice.as_ref().split_at(3);
}

#[test]
fn test_split_at_empty_slice() {
    let slice = Box::new(Slice { entries: [] });

    let (first, second) = slice.as_ref().split_at(0);
    assert_eq!(first.len(), 0);
    assert_eq!(second.len(), 0);
}

