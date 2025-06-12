// Answer 0

#[test]
fn test_get_range_valid_full_range() {
    let entries = Box::new(Slice {
        entries: [
            Bucket { hash: HashValue::from(1), key: 1, value: "a" },
            Bucket { hash: HashValue::from(2), key: 2, value: "b" },
            Bucket { hash: HashValue::from(3), key: 3, value: "c" },
        ],
    });

    let result = entries.get_range(0..3);
}

#[test]
fn test_get_range_valid_single_element() {
    let entries = Box::new(Slice {
        entries: [
            Bucket { hash: HashValue::from(1), key: 1, value: "a" },
        ],
    });

    let result = entries.get_range(0..1);
}

#[test]
fn test_get_range_empty_slice() {
    let entries = Box::new(Slice {
        entries: [],
    });

    let result = entries.get_range(0..0);
}

#[test]
fn test_get_range_invalid_below_zero() {
    let entries = Box::new(Slice {
        entries: [
            Bucket { hash: HashValue::from(1), key: 1, value: "a" },
        ],
    });

    let result = entries.get_range(..0);
}

#[test]
fn test_get_range_invalid_above_length() {
    let entries = Box::new(Slice {
        entries: [
            Bucket { hash: HashValue::from(1), key: 1, value: "a" },
            Bucket { hash: HashValue::from(2), key: 2, value: "b" },
        ],
    });

    let result = entries.get_range(0..3);
}

#[test]
fn test_get_range_valid_inclusive_range() {
    let entries = Box::new(Slice {
        entries: [
            Bucket { hash: HashValue::from(1), key: 1, value: "a" },
            Bucket { hash: HashValue::from(2), key: 2, value: "b" },
        ],
    });

    let result = entries.get_range(0..=1);
}

#[test]
fn test_get_range_valid_exclusive_start() {
    let entries = Box::new(Slice {
        entries: [
            Bucket { hash: HashValue::from(1), key: 1, value: "a" },
            Bucket { hash: HashValue::from(2), key: 2, value: "b" },
            Bucket { hash: HashValue::from(3), key: 3, value: "c" },
        ],
    });

    let result = entries.get_range(1..3);
}

