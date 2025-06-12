// Answer 0

#[test]
fn test_fmt_debug_iter_empty() {
    struct TestKeys<'a, K> {
        _marker: std::marker::PhantomData<&'a K>,
    }

    impl<'a, K: fmt::Debug> fmt::Debug for TestKeys<'a, K> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_list().finish()
        }
    }

    let keys = TestKeys { _marker: std::marker::PhantomData };
    let iter = Iter { iter: keys };
    let result = format!("{:?}", iter);
    assert_eq!(result, "[]");
}

#[test]
fn test_fmt_debug_iter_with_elements() {
    struct TestKeys<'a> {
        elements: &'a [i32],
        index: usize,
    }

    impl<'a> fmt::Debug for TestKeys<'a> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut d = f.debug_list();
            for &element in self.elements {
                d.entry(&element);
            }
            d.finish()
        }
    }

    let elements = &[1, 2, 3];
    let keys = TestKeys { elements, index: 0 };
    let iter = Iter { iter: keys };
    let result = format!("{:?}", iter);
    assert_eq!(result, "[1, 2, 3]");
}

