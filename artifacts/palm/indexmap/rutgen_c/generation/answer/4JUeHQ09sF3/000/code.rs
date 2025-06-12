// Answer 0

#[test]
fn test_into_keys_debug_fmt() {
    #[derive(Debug)]
    struct TestKey(u32);
    #[derive(Debug)]
    struct TestValue(u32);
    
    let buckets = vec![
        Bucket { hash: 1, key: TestKey(1), value: TestValue(10) },
        Bucket { hash: 2, key: TestKey(2), value: TestValue(20) },
        Bucket { hash: 3, key: TestKey(3), value: TestValue(30) },
    ];
    
    let iter = IntoKeys { iter: buckets.into_iter() };
    
    let mut output = Vec::new();
    let result = fmt::write(&mut output, format_args!("{:?}", iter));
    
    assert!(result.is_ok());
    let expected_output = "[TestKey(1), TestKey(2), TestKey(3)]";
    assert_eq!(String::from_utf8(output).unwrap(), expected_output);
}

#[test]
fn test_into_keys_debug_fmt_empty() {
    #[derive(Debug)]
    struct TestKey(u32);
    
    let buckets: Vec<Bucket<TestKey, ()>> = vec![];
    let iter = IntoKeys { iter: buckets.into_iter() };
    
    let mut output = Vec::new();
    let result = fmt::write(&mut output, format_args!("{:?}", iter));
    
    assert!(result.is_ok());
    let expected_output = "[]";
    assert_eq!(String::from_utf8(output).unwrap(), expected_output);
}

