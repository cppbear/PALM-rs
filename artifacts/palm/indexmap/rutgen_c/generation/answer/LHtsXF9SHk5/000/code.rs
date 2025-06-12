// Answer 0

#[test]
fn test_values_new_with_empty_slice() {
    let entries: &[Bucket<i32, &str>] = &[];
    let values = Values::new(entries);
    assert_eq!(values.iter.len(), 0);
}

#[test]
fn test_values_new_with_non_empty_slice() {
    let bucket1 = Bucket { hash: 1, key: 1, value: "value1" };
    let bucket2 = Bucket { hash: 2, key: 2, value: "value2" };
    let entries = &[bucket1, bucket2];
    let values = Values::new(entries);
    assert_eq!(values.iter.len(), 2);
}

