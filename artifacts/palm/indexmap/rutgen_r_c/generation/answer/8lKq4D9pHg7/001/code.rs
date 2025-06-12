// Answer 0

#[test]
fn test_split_at_mut_valid_index() {
    let mut slice = Slice {
        entries: [
            Bucket { hash: 0, key: 1, value: "a" },
            Bucket { hash: 1, key: 2, value: "b" },
            Bucket { hash: 2, key: 3, value: "c" },
        ],
    };
    let (first, second) = slice.split_at_mut(2);
    assert_eq!(first.len(), 2);
    assert_eq!(second.len(), 1);
}

#[test]
#[should_panic]
fn test_split_at_mut_index_out_of_bounds() {
    let mut slice = Slice {
        entries: [
            Bucket { hash: 0, key: 1, value: "a" },
            Bucket { hash: 1, key: 2, value: "b" },
        ],
    };
    // This will cause a panic since index 3 is out of bounds.
    slice.split_at_mut(3);
}

#[test]
fn test_split_at_mut_empty_slice() {
    let mut slice = Slice {
        entries: [],
    };
    let (first, second) = slice.split_at_mut(0);
    assert_eq!(first.len(), 0);
    assert_eq!(second.len(), 0);
}

#[test]
fn test_split_at_mut_single_element() {
    let mut slice = Slice {
        entries: [
            Bucket { hash: 0, key: 1, value: "only" },
        ],
    };
    let (first, second) = slice.split_at_mut(1);
    assert_eq!(first.len(), 1);
    assert_eq!(second.len(), 0);
}

