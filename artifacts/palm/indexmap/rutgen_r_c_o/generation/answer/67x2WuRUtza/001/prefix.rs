// Answer 0

#[test]
fn test_split_first_non_empty() {
    let bucket = Bucket { hash: 0, key: "key1", value: "value1" };
    let slice = Slice { entries: [bucket] };
    let result = slice.split_first();
}

#[test]
fn test_split_first_empty() {
    let slice = Slice { entries: [] };
    let result = slice.split_first();
}

#[test]
fn test_split_first_two_elements() {
    let bucket1 = Bucket { hash: 0, key: "key1", value: "value1" };
    let bucket2 = Bucket { hash: 1, key: "key2", value: "value2" };
    let slice = Slice { entries: [bucket1, bucket2] };
    let result = slice.split_first();
}

