// Answer 0

#[test]
fn test_split_last_non_empty() {
    let bucket1 = Bucket { hash: 1, key: "key1", value: "value1" };
    let bucket2 = Bucket { hash: 2, key: "key2", value: "value2" };
    let entries = vec![bucket1, bucket2];
    let slice = Slice::from_slice(&entries);
    let result = slice.split_last();
}

#[test]
fn test_split_last_single_element() {
    let bucket = Bucket { hash: 1, key: "key1", value: "value1" };
    let entries = vec![bucket];
    let slice = Slice::from_slice(&entries);
    let result = slice.split_last();
}

#[test]
fn test_split_last_empty() {
    let entries: Vec<Bucket<&str, &str>> = vec![];
    let slice = Slice::from_slice(&entries);
    let result = slice.split_last();
}

#[test]
fn test_split_last_two_elements() {
    let bucket1 = Bucket { hash: 1, key: "key1", value: "value1" };
    let bucket2 = Bucket { hash: 2, key: "key2", value: "value2" };
    let entries = vec![bucket1, bucket2];
    let slice = Slice::from_slice(&entries);
    let result = slice.split_last();
}

#[test]
fn test_split_last_large_number_of_elements() {
    let mut entries = Vec::new();
    for i in 0..100 {
        entries.push(Bucket { hash: i, key: format!("key{}", i), value: format!("value{}", i) });
    }
    let slice = Slice::from_slice(&entries);
    let result = slice.split_last();
}

