// Answer 0

#[test]
fn test_fmt_with_debug_iter() {
    struct TestKey;
    struct TestValue;

    impl Debug for TestKey {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "TestKey")
        }
    }

    impl Debug for TestValue {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "TestValue")
        }
    }

    let items: Vec<(TestKey, TestValue)> = vec![(TestKey, TestValue)];
    let raw_iter = RawIter {
        iter: RawIterRange { items: &items, index: 0, end: 1 },
        items: items.len(),
    };
    let iter = Iter {
        inner: raw_iter,
        marker: PhantomData,
    };

    let mut buffer = String::new();
    let result = write!(buffer, "{:?}", iter);
    
    assert!(result.is_ok());
    assert_eq!(buffer, "TestKey\nTestValue\n");
}

#[test]
#[should_panic]
fn test_fmt_with_empty_iter_should_panic() {
    struct TestKey;
    struct TestValue;

    impl Debug for TestKey {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "TestKey")
        }
    }

    impl Debug for TestValue {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "TestValue")
        }
    }

    let items: Vec<(TestKey, TestValue)> = vec![];
    let raw_iter = RawIter {
        iter: RawIterRange { items: &items, index: 0, end: 0 },
        items: items.len(),
    };
    let iter = Iter {
        inner: raw_iter,
        marker: PhantomData,
    };

    let _ = write!(String::new(), "{:?}", iter);
}

