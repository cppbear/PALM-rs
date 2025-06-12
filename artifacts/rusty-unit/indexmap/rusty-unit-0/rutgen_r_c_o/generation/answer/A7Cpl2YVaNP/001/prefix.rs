// Answer 0

#[test]
fn test_into_boxed_small() {
    let entries = vec![
        Bucket { hash: HashValue::new(), key: 1, value: "value1" },
        Bucket { hash: HashValue::new(), key: 2, value: "value2" },
    ];
    let boxed_slice = Box::new(Slice { entries: entries.as_slice().to_vec().try_into().unwrap() });
    let _result = boxed_slice.into_boxed();
}

#[test]
fn test_into_boxed_large() {
    let entries: Vec<Bucket<usize, &str>> = (0..100000).map(|i| Bucket { hash: HashValue::new(), key: i, value: "value" }).collect();
    let boxed_slice = Box::new(Slice { entries: entries.try_into().unwrap() });
    let _result = boxed_slice.into_boxed();
}

#[test]
#[should_panic]
fn test_into_boxed_empty() {
    let entries: Vec<Bucket<usize, &str>> = Vec::new();
    let boxed_slice = Box::new(Slice { entries: entries.try_into().unwrap() });
    let _result = boxed_slice.into_boxed();
}

