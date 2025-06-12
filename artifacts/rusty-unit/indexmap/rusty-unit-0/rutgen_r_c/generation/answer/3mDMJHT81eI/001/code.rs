// Answer 0

#[test]
fn test_values_mut_empty_slice() {
    let mut slice: Slice<u32, String> = Slice::new_mut();
    let mut values_iter = slice.values_mut();
    assert!(values_iter.iter.len() == 0);
}

#[test]
fn test_values_mut_single_element() {
    let mut slice: Slice<u32, String> = Slice {
        entries: [Bucket { hash: 0, key: 1, value: String::from("one") }],
    };
    let mut values_iter = slice.values_mut();
    assert_eq!(values_iter.iter.len(), 1);
    assert_eq!(values_iter.iter.next().unwrap().value, "one");
}

#[test]
fn test_values_mut_multiple_elements() {
    let mut slice: Slice<u32, String> = Slice {
        entries: [
            Bucket { hash: 0, key: 1, value: String::from("one") },
            Bucket { hash: 0, key: 2, value: String::from("two") },
        ],
    };
    let mut values_iter = slice.values_mut();
    let mut values: Vec<String> = values_iter.iter.map(|b| b.value).collect();
    assert_eq!(values, vec![String::from("one"), String::from("two")]);
}

#[test]
fn test_values_mut_after_modification() {
    let mut slice: Slice<u32, String> = Slice {
        entries: [
            Bucket { hash: 0, key: 1, value: String::from("one") },
            Bucket { hash: 0, key: 2, value: String::from("two") },
        ],
    };
    {
        let mut values_iter = slice.values_mut();
        if let Some(bucket) = values_iter.iter.next() {
            bucket.value = String::from("changed");
        }
    }
    let mut values_iter = slice.values_mut();
    let values: Vec<String> = values_iter.iter.map(|b| b.value).collect();
    assert_eq!(values, vec![String::from("changed"), String::from("two")]);
}

