// Answer 0

#[test]
fn test_keys_debug_format() {
    #[derive(Debug, Clone)]
    struct TestKey(i32);
    #[derive(Debug, Clone)]
    struct TestValue(i32);

    struct TestKeys<'a> {
        inner: Iter<'a, TestKey, TestValue>,
    }

    impl<'a> TestKeys<'a> {
        fn new() -> Self {
            let keys = vec![TestKey(1), TestKey(2)];
            let values = vec![TestValue(10), TestValue(20)];
            let inner = Iter {
                inner: RawIter::new(keys.into_iter().zip(values.into_iter())),
                marker: PhantomData,
            };
            TestKeys { inner }
        }
    }

    let test_keys = TestKeys::new();
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{:?}", test_keys);

    assert!(result.is_ok());
    assert!(buffer.contains("TestKey(1)"));
    assert!(buffer.contains("TestKey(2)"));
    assert!(buffer.contains("TestValue(10)"));
    assert!(buffer.contains("TestValue(20)"));
}

#[test]
#[should_panic]
fn test_keys_debug_format_empty() {
    #[derive(Debug, Clone)]
    struct TestKey(i32);
    #[derive(Debug, Clone)]
    struct TestValue(i32);

    struct EmptyKeys<'a> {
        inner: Iter<'a, TestKey, TestValue>,
    }

    impl<'a> EmptyKeys<'a> {
        fn new() -> Self {
            let inner = Iter {
                inner: RawIter::new(vec![].into_iter()),
                marker: PhantomData,
            };
            EmptyKeys { inner }
        }
    }

    let empty_keys = EmptyKeys::new();
    let mut buffer = String::new();
    let _result = write!(&mut buffer, "{:?}", empty_keys);
}

