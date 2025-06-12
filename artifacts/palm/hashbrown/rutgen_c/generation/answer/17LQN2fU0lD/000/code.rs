// Answer 0

#[test]
fn test_keys_debug_fmt() {
    use std::fmt::{self, Debug};
    use std::vec::Vec;

    struct TestKeys<'a> {
        // Assuming K is of type i32 for testing
        inner: Iter<'a, i32, ()>,
    }

    impl<'a> Clone for TestKeys<'a> {
        fn clone(&self) -> Self {
            TestKeys {
                inner: self.inner.clone(),
            }
        }
    }

    impl<'a> Debug for TestKeys<'a> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_list().entries(self.clone()).finish()
        }
    }

    let keys = TestKeys {
        inner: Iter {
            inner: RawIter::new(vec![(1, ()), (2, ())].into_iter()),
            marker: PhantomData,
        },
    };

    let output = format!("{:?}", keys);
    assert!(output.contains("1"));
    assert!(output.contains("2"));
}

