// Answer 0

#[test]
fn test_slice_debug_empty() {
    let slice: Slice<i32> = Slice { entries: [] };
    let result = format!("{:?}", slice);
    assert_eq!(result, "[]");
}

#[test]
fn test_slice_debug_single_element() {
    let bucket = Bucket { hash: 0, key: 1, value: "value" };
    let slice: Slice<(i32, &str)> = Slice { entries: [bucket] };
    let result = format!("{:?}", slice);
    assert_eq!(result, "[1: value]");
}

#[test]
fn test_slice_debug_multiple_elements() {
    let bucket1 = Bucket { hash: 0, key: 1, value: "value1" };
    let bucket2 = Bucket { hash: 1, key: 2, value: "value2" };
    let slice: Slice<(i32, &str)> = Slice { entries: [bucket1, bucket2] };
    let result = format!("{:?}", slice);
    assert_eq!(result, "[1: value1, 2: value2]");
}

