// Answer 0

#[test]
fn test_index_valid_input_0() {
    let bucket = Bucket { hash: 0, key: "key0", value: "value0" };
    let slice = Slice { entries: [bucket] };
    let _result = slice.index(0);
}

#[test]
fn test_index_valid_input_1() {
    let bucket1 = Bucket { hash: 1, key: "key1", value: "value1" };
    let bucket2 = Bucket { hash: 2, key: "key2", value: "value2" };
    let slice = Slice { entries: [bucket1, bucket2] };
    let _result = slice.index(1);
}

#[test]
#[should_panic]
fn test_index_out_of_bounds() {
    let bucket = Bucket { hash: 0, key: "key", value: "value" };
    let slice = Slice { entries: [bucket] };
    let _result = slice.index(1); // out of bounds
}

#[test]
#[should_panic]
fn test_index_negative() {
    let slice = Slice { entries: [] };
    let _result = slice.index(!0); // simulating a negative index
}

#[test]
#[should_panic]
fn test_index_large_index() {
    let bucket = Bucket { hash: 0, key: "key", value: "value" };
    let slice = Slice { entries: [bucket] };
    let _result = slice.index(usize::MAX); // out of bounds
}

