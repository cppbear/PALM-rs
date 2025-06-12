// Answer 0

#[test]
fn test_split_first_non_empty() {
    let entries: [Bucket<i32>; 3] = [
        Bucket { hash: 1, key: 10, value: "value1" },
        Bucket { hash: 2, key: 20, value: "value2" },
        Bucket { hash: 3, key: 30, value: "value3" },
    ];
    let slice = Slice::from_slice(&entries);
    let result = slice.split_first();
}

#[test]
fn test_split_first_single_element() {
    let entries: [Bucket<i32>; 1] = [
        Bucket { hash: 1, key: 10, value: "value1" },
    ];
    let slice = Slice::from_slice(&entries);
    let result = slice.split_first();
}

#[test]
fn test_split_first_multiple_elements() {
    let entries: [Bucket<i32>; 5] = [
        Bucket { hash: 1, key: 10, value: "value1" },
        Bucket { hash: 2, key: 20, value: "value2" },
        Bucket { hash: 3, key: 30, value: "value3" },
        Bucket { hash: 4, key: 40, value: "value4" },
        Bucket { hash: 5, key: 50, value: "value5" },
    ];
    let slice = Slice::from_slice(&entries);
    let result = slice.split_first();
}

#[test]
#[should_panic]
fn test_split_first_empty() {
    let entries: [Bucket<i32>; 0] = [];
    let slice = Slice::from_slice(&entries);
    let result = slice.split_first();
}

