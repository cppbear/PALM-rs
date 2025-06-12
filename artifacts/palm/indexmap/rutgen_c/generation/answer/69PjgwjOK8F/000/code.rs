// Answer 0

#[test]
fn test_into_values() {
    struct TestBucket {
        hash: u64,
        key: i32,
        value: String,
    }

    let entries = vec![
        TestBucket { hash: 1, key: 1, value: "value1".to_string() },
        TestBucket { hash: 2, key: 2, value: "value2".to_string() },
    ];

    let slice = Box::new(Slice {
        entries: entries.clone(),
    });

    let into_values_result = slice.into_values();

    let mut values_iterator = into_values_result.iter;

    assert_eq!(values_iterator.next().unwrap().value, "value1");
    assert_eq!(values_iterator.next().unwrap().value, "value2");
}

#[test]
fn test_into_values_empty() {
    let empty_slice = Box::new(Slice {
        entries: vec![],
    });

    let into_values_result = empty_slice.into_values();

    let mut values_iterator = into_values_result.iter;

    assert!(values_iterator.next().is_none());
}

