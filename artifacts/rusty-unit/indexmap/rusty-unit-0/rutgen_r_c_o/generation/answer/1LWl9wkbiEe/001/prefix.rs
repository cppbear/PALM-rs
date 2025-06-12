// Answer 0

#[test]
fn test_get_index_valid_zero() {
    let entries = vec![
        Bucket { hash: 0, key: "key1", value: "value1" },
    ];
    let slice = Slice { entries: entries.try_into().unwrap() };
    let _ = slice.get_index(0);
}

#[test]
fn test_get_index_valid_last() {
    let entries = vec![
        Bucket { hash: 0, key: "key1", value: "value1" },
        Bucket { hash: 1, key: "key2", value: "value2" },
    ];
    let slice = Slice { entries: entries.try_into().unwrap() };
    let _ = slice.get_index(1);
}

#[test]
fn test_get_index_out_of_bounds_high() {
    let entries = vec![
        Bucket { hash: 0, key: "key1", value: "value1" },
    ];
    let slice = Slice { entries: entries.try_into().unwrap() };
    let _ = slice.get_index(1);
}

#[test]
fn test_get_index_out_of_bounds_low() {
    let entries = vec![
        Bucket { hash: 0, key: "key1", value: "value1" },
    ];
    let slice = Slice { entries: entries.try_into().unwrap() };
    let _ = slice.get_index(usize::MAX);
}

