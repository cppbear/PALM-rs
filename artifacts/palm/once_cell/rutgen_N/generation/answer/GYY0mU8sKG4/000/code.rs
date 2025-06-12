// Answer 0

#[test]
fn test_get_some() {
    struct Inner {
        value: usize,
    }
    
    impl Inner {
        fn get(&self) -> Option<usize> {
            Some(self.value)
        }
    }
    
    struct TestStruct {
        inner: Inner,
    }

    impl TestStruct {
        fn from_usize(value: usize) -> bool {
            value != 0
        }

        fn get(&self) -> Option<bool> {
            self.inner.get().map(Self::from_usize)
        }
    }

    let test_instance = TestStruct { inner: Inner { value: 1 } };
    assert_eq!(test_instance.get(), Some(true));
}

#[test]
fn test_get_none() {
    struct Inner {
        value: usize,
    }
    
    impl Inner {
        fn get(&self) -> Option<usize> {
            None
        }
    }
    
    struct TestStruct {
        inner: Inner,
    }

    impl TestStruct {
        fn from_usize(value: usize) -> bool {
            value != 0
        }

        fn get(&self) -> Option<bool> {
            self.inner.get().map(Self::from_usize)
        }
    }

    let test_instance = TestStruct { inner: Inner { value: 0 } };
    assert_eq!(test_instance.get(), None);
}

