// Answer 0

#[test]
fn test_into_keys_with_power_of_two_entries() {
    let entries = vec![
        Bucket { hash: HashValue::default(), key: 1, value: "a" },
        Bucket { hash: HashValue::default(), key: 2, value: "b" },
        Bucket { hash: HashValue::default(), key: 3, value: "c" },
        Bucket { hash: HashValue::default(), key: 4, value: "d" },
    ];
    let slice = Box::new(Slice { entries });
    let keys = slice.into_keys();
}

#[test]
fn test_into_keys_with_large_power_of_two_entries() {
    let entries: Vec<Bucket<u32, &str>> = (1..=1024).map(|i| Bucket { hash: HashValue::default(), key: i, value: "value" }).collect();
    let slice = Box::new(Slice { entries });
    let keys = slice.into_keys();
}

#[test]
fn test_into_keys_with_exactly_one_entry() {
    let entries = vec![Bucket { hash: HashValue::default(), key: 1, value: "a" }];
    let slice = Box::new(Slice { entries });
    let keys = slice.into_keys();
}

#[test]
fn test_into_keys_with_maximum_power_of_two_entries() {
    let entries: Vec<Bucket<u32, &str>> = (1..=8192).map(|i| Bucket { hash: HashValue::default(), key: i, value: "value" }).collect();
    let slice = Box::new(Slice { entries });
    let keys = slice.into_keys();
}

#[test]
fn test_into_keys_with_empty_slice() {
    let entries: Vec<Bucket<u32, &str>> = vec![];
    let slice = Box::new(Slice { entries });
    let keys = slice.into_keys();
}

