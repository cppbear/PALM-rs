// Answer 0

#[test]
fn test_fmt_with_non_empty_iter() {
    struct TestKey {
        id: usize,
    }

    struct TestValue {
        data: String,
    }

    impl Bucket<TestKey, TestValue> {
        fn value_ref(&self) -> &TestValue {
            &self.value
        }
    }

    let buckets = vec![
        Bucket { hash: HashValue::default(), key: TestKey { id: 1 }, value: TestValue { data: "value1".to_string() } },
        Bucket { hash: HashValue::default(), key: TestKey { id: 2 }, value: TestValue { data: "value2".to_string() } },
    ];

    let iter = IntoValues { iter: buckets.into_iter() };
    let mut output = Vec::new();
    let result = fmt::Debug::fmt(&iter, &mut output);

    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "[value1, value2]");
}

#[test]
fn test_fmt_with_empty_iter() {
    struct TestKey {
        id: usize,
    }

    struct TestValue {
        data: String,
    }

    impl Bucket<TestKey, TestValue> {
        fn value_ref(&self) -> &TestValue {
            &self.value
        }
    }

    let buckets: Vec<Bucket<TestKey, TestValue>> = vec![];
    let iter = IntoValues { iter: buckets.into_iter() };
    let mut output = Vec::new();
    let result = fmt::Debug::fmt(&iter, &mut output);

    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "[]");
}

#[test]
#[should_panic]
fn test_fmt_with_invalid_data() {
    struct InvalidKey;
    struct InvalidValue;

    impl Bucket<InvalidKey, InvalidValue> {
        fn value_ref(&self) -> &InvalidValue {
            panic!("This should panic due to invalid data!");
        }
    }

    let buckets = vec![Bucket { hash: HashValue::default(), key: InvalidKey, value: InvalidValue }];
    let iter = IntoValues { iter: buckets.into_iter() };
    let mut output = Vec::new();
    let _ = fmt::Debug::fmt(&iter, &mut output);
}

