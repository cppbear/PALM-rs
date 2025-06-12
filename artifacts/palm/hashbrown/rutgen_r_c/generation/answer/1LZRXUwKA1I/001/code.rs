// Answer 0

#[test]
fn test_iter_fmt() {
    struct TestKeys<'a, K> {
        inner: Iter<'a, K>,
    }

    impl<'a, K: fmt::Debug> fmt::Debug for TestKeys<'a, K> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_list().entries(self.inner.clone()).finish()
        }
    }

    let keys = vec![1, 2, 3];
    let iter = Iter {
        iter: TestKeys { inner: Iter { iter: Keys { inner: iter.clone() } } },
    };

    let result = format!("{:?}", iter);
    assert!(result.contains("1"));
    assert!(result.contains("2"));
    assert!(result.contains("3"));
}

#[test]
fn test_iter_fmt_empty() {
    struct TestKeys<'a, K> {
        inner: Iter<'a, K>,
    }

    impl<'a, K: fmt::Debug> fmt::Debug for TestKeys<'a, K> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_list().entries(self.inner.clone()).finish()
        }
    }

    let keys: Vec<i32> = vec![];
    let iter = Iter {
        iter: TestKeys { inner: Iter { iter: Keys { inner: iter.clone() } } },
    };

    let result = format!("{:?}", iter);
    assert_eq!(result, "[]");
}

