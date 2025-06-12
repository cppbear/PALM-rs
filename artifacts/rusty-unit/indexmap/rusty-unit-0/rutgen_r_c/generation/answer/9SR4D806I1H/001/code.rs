// Answer 0

#[test]
fn test_into_iter_debug_with_single_element() {
    let bucket = Bucket {
        hash: HashValue::new(1),
        key: "key1",
        value: "value1",
    };
    let iter = IntoIter {
        iter: vec![bucket].into_iter(),
    };
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", iter);
    
    assert!(result.is_ok());
    assert_eq!(output, "[\"key1\"]\n");
}

#[test]
fn test_into_iter_debug_with_multiple_elements() {
    let buckets = vec![
        Bucket {
            hash: HashValue::new(1),
            key: "key1",
            value: "value1",
        },
        Bucket {
            hash: HashValue::new(2),
            key: "key2",
            value: "value2",
        },
    ];
    let iter = IntoIter {
        iter: buckets.into_iter(),
    };
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", iter);
    
    assert!(result.is_ok());
    assert_eq!(output, "[\"key1\", \"key2\"]\n");
}

#[test]
fn test_into_iter_debug_with_empty() {
    let iter = IntoIter {
        iter: vec![].into_iter(),
    };
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", iter);

    assert!(result.is_ok());
    assert_eq!(output, "[]\n");
}

