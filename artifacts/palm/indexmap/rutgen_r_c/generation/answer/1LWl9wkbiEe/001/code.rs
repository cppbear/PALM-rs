// Answer 0

#[test]
fn test_get_index_valid() {
    let bucket1 = Bucket { hash: 1, key: 1, value: "value1" };
    let bucket2 = Bucket { hash: 2, key: 2, value: "value2" };
    let bucket3 = Bucket { hash: 3, key: 3, value: "value3" };
    let entries = vec![bucket1, bucket2, bucket3];
    let slice = Slice { entries: entries.clone() };

    assert_eq!(slice.get_index(0), Some((&1, &"value1")));
    assert_eq!(slice.get_index(1), Some((&2, &"value2")));
    assert_eq!(slice.get_index(2), Some((&3, &"value3")));
}

#[test]
fn test_get_index_out_of_bounds() {
    let bucket1 = Bucket { hash: 1, key: 1, value: "value1" };
    let entries = vec![bucket1];
    let slice = Slice { entries };

    assert_eq!(slice.get_index(1), None); // Out of bounds
    assert_eq!(slice.get_index(100), None); // Far out of bounds
}

#[test]
fn test_get_index_empty() {
    let entries: Vec<Bucket<u32, &str>> = vec![];
    let slice = Slice { entries };

    assert_eq!(slice.get_index(0), None); // Empty slice should return None
}

