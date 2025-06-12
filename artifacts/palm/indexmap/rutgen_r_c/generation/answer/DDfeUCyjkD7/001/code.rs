// Answer 0

#[test]
fn test_values_debug_format_with_empty_values() {
    let empty_values: Values<usize, usize> = Values { iter: [].iter() };
    let result = format!("{:?}", empty_values);
    assert_eq!(result, "[]");
}

#[test]
fn test_values_debug_format_with_single_element() {
    let single_bucket = Bucket { hash: HashValue::default(), key: 1, value: 10 };
    let values: Values<usize, usize> = Values { iter: vec![single_bucket].iter() };
    let result = format!("{:?}", values);
    assert_eq!(result, "[1: 10]");
}

#[test]
fn test_values_debug_format_with_multiple_elements() {
    let buckets = vec![
        Bucket { hash: HashValue::default(), key: 1, value: 10 },
        Bucket { hash: HashValue::default(), key: 2, value: 20 },
        Bucket { hash: HashValue::default(), key: 3, value: 30 },
    ];
    let values: Values<usize, usize> = Values { iter: buckets.iter() };
    let result = format!("{:?}", values);
    assert_eq!(result, "[1: 10, 2: 20, 3: 30]");
}

