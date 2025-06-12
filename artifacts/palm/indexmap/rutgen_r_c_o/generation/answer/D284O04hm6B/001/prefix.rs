// Answer 0

#[test]
fn test_index_mut_with_valid_index() {
    let mut slice = Slice {
        entries: [
            Bucket { hash: 0, key: 1, value: 10 },
            Bucket { hash: 0, key: 2, value: 20 },
            Bucket { hash: 0, key: 3, value: 30 },
        ],
    };
    let _ = slice.index_mut(1);
}

#[test]
fn test_index_mut_with_zero_index() {
    let mut slice = Slice {
        entries: [
            Bucket { hash: 0, key: 1, value: 10 },
            Bucket { hash: 0, key: 2, value: 20 },
            Bucket { hash: 0, key: 3, value: 30 },
        ],
    };
    let _ = slice.index_mut(0);
}

#[test]
fn test_index_mut_with_last_index() {
    let mut slice = Slice {
        entries: [
            Bucket { hash: 0, key: 1, value: 10 },
            Bucket { hash: 0, key: 2, value: 20 },
            Bucket { hash: 0, key: 3, value: 30 },
        ],
    };
    let _ = slice.index_mut(2);
}

#[should_panic]
fn test_index_mut_with_out_of_bounds_index() {
    let mut slice = Slice {
        entries: [
            Bucket { hash: 0, key: 1, value: 10 },
            Bucket { hash: 0, key: 2, value: 20 },
            Bucket { hash: 0, key: 3, value: 30 },
        ],
    };
    let _ = slice.index_mut(3);
}

#[should_panic]
fn test_index_mut_with_negative_index() {
    let mut slice = Slice {
        entries: [
            Bucket { hash: 0, key: 1, value: 10 },
            Bucket { hash: 0, key: 2, value: 20 },
            Bucket { hash: 0, key: 3, value: 30 },
        ],
    };
    let _ = slice.index_mut(usize::MAX);
}

