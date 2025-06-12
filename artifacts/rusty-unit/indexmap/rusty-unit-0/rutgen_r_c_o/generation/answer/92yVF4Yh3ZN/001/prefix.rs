// Answer 0

#[test]
fn test_split_last_non_empty() {
    let entries = vec![
        Bucket { hash: 0, key: 1, value: "a" },
        Bucket { hash: 0, key: 2, value: "b" },
        Bucket { hash: 0, key: 3, value: "c" },
    ];
    let slice = Slice::from_slice(&entries);
    let result = slice.split_last();
}

#[test]
fn test_split_last_single_element() {
    let entries = vec![
        Bucket { hash: 0, key: 1, value: "a" },
    ];
    let slice = Slice::from_slice(&entries);
    let result = slice.split_last();
}

#[test]
fn test_split_last_with_two_elements() {
    let entries = vec![
        Bucket { hash: 0, key: 10, value: "x" },
        Bucket { hash: 0, key: 20, value: "y" },
    ];
    let slice = Slice::from_slice(&entries);
    let result = slice.split_last();
}

#[test]
fn test_split_last_multiple_elements() {
    let entries = vec![
        Bucket { hash: 0, key: 5, value: "one" },
        Bucket { hash: 0, key: 10, value: "two" },
        Bucket { hash: 0, key: 15, value: "three" },
        Bucket { hash: 0, key: 20, value: "four" },
    ];
    let slice = Slice::from_slice(&entries);
    let result = slice.split_last();
}

#[test]
fn test_split_last_empty() {
    let entries: Vec<Bucket<i32, &str>> = vec![];
    let slice = Slice::from_slice(&entries);
    let result = slice.split_last();
}

