// Answer 0

#[test]
fn test_index_with_valid_zero_index() {
    let bucket = Bucket { hash: HashValue::default(), key: 42, value: "value1" };
    let slice = Slice { entries: [bucket] };
    let _result = slice.index(0);
}

#[test]
fn test_index_with_valid_middle_index() {
    let bucket1 = Bucket { hash: HashValue::default(), key: 42, value: "value1" };
    let bucket2 = Bucket { hash: HashValue::default(), key: 43, value: "value2" };
    let slice = Slice { entries: [bucket1, bucket2] };
    let _result = slice.index(1);
}

#[test]
fn test_index_with_valid_last_index() {
    let bucket1 = Bucket { hash: HashValue::default(), key: 100, value: "value1" };
    let bucket2 = Bucket { hash: HashValue::default(), key: 200, value: "value2" };
    let slice = Slice { entries: [bucket1, bucket2] };
    let _result = slice.index(1);
}

#[should_panic]
fn test_index_with_beyond_last_index() {
    let bucket = Bucket { hash: HashValue::default(), key: 42, value: "value1" };
    let slice = Slice { entries: [bucket] };
    let _result = slice.index(1);
}

#[should_panic]
fn test_index_with_negative_index() {
    let slice = Slice { entries: [] };
    let _result = slice.index(usize::MAX);
}

