// Answer 0

#[test]
fn test_iter_mut_debug() {
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

    let key_value_pairs = vec![(TestKey, TestValue)];
    let raw_iter = RawIter {
        iter: RawIterRange { /* initialize with appropriate parameters */ },
        items: key_value_pairs.len(),
    };

    let iter_mut = IterMut {
        inner: raw_iter,
        marker: PhantomData,
    };

    let mut output = String::new();
    {
        let mut formatter = fmt::Formatter::new(&mut output);
        let _ = iter_mut.fmt(&mut formatter);
    }

    assert_eq!(output, "TestKey, TestValue");
}

