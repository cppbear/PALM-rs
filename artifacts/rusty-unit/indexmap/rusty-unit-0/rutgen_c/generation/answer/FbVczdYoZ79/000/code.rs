// Answer 0

#[test]
fn test_binary_search_by_found() {
    let slice = Slice {
        entries: [
            Bucket { hash: 0, key: 1, value: "a" },
            Bucket { hash: 0, key: 2, value: "b" },
            Bucket { hash: 0, key: 3, value: "c" },
        ],
    };

    let result = slice.binary_search_by(|&key, &value| {
        if key == 2 {
            Ordering::Equal
        } else if key < 2 {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });
    assert_eq!(result, Ok(1));
}

#[test]
fn test_binary_search_by_insert_position() {
    let slice = Slice {
        entries: [
            Bucket { hash: 0, key: 1, value: "a" },
            Bucket { hash: 0, key: 3, value: "c" },
        ],
    };

    let result = slice.binary_search_by(|&key, &value| {
        if key == 2 {
            Ordering::Equal
        } else if key < 2 {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });
    assert_eq!(result, Err(1));
}

#[test]
fn test_binary_search_by_empty() {
    let slice = Slice {
        entries: [],
    };

    let result = slice.binary_search_by(|&key, &value| {
        Ordering::Greater // always greater than anything since the slice is empty
    });
    assert_eq!(result, Err(0));
}

