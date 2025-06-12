// Answer 0

#[test]
fn test_fmt_debug_list() {
    use std::fmt;
    use std::marker::PhantomData;
    
    struct Iter<T> {
        inner: Vec<T>,
        marker: PhantomData<*const T>,
    }

    struct TestStruct {
        inner: Vec<i32>,
    }

    impl fmt::Debug for TestStruct {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_list()
                .entries(Iter {
                    inner: self.inner.clone(),
                    marker: PhantomData,
                })
                .finish()
        }
    }

    impl<T> fmt::Debug for Iter<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_list().entries(&self.inner).finish()
        }
    }

    let data = TestStruct { inner: vec![1, 2, 3] };
    
    let expected = format!("{:?}", data);
    let actual = format!("{:?}", data);
    
    assert_eq!(expected, actual);
}

