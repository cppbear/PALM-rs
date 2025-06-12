// Answer 0

#[test]
fn test_get_with_some_value() {
    struct TestStruct {
        inner: once_cell::unsync::OnceCell<usize>,
    }

    impl TestStruct {
        fn from_usize(value: usize) -> Option<bool> {
            Some(value != 0)
        }
        
        fn new() -> Self {
            TestStruct {
                inner: once_cell::unsync::OnceCell::new(),
            }
        }
        
        fn set_value(&self, value: usize) {
            self.inner.set(value).ok();
        }
        
        pub fn get(&self) -> Option<bool> {
            self.inner.get().map(Self::from_usize)
        }
    }

    let test_struct = TestStruct::new();
    test_struct.set_value(1);
    assert_eq!(test_struct.get(), Some(true));
}

#[test]
fn test_get_with_zero_value() {
    struct TestStruct {
        inner: once_cell::unsync::OnceCell<usize>,
    }

    impl TestStruct {
        fn from_usize(value: usize) -> Option<bool> {
            Some(value != 0)
        }
        
        fn new() -> Self {
            TestStruct {
                inner: once_cell::unsync::OnceCell::new(),
            }
        }
        
        fn set_value(&self, value: usize) {
            self.inner.set(value).ok();
        }
        
        pub fn get(&self) -> Option<bool> {
            self.inner.get().map(Self::from_usize)
        }
    }

    let test_struct = TestStruct::new();
    test_struct.set_value(0);
    assert_eq!(test_struct.get(), Some(false));
}

#[test]
fn test_get_without_value() {
    struct TestStruct {
        inner: once_cell::unsync::OnceCell<usize>,
    }

    impl TestStruct {
        fn from_usize(value: usize) -> Option<bool> {
            Some(value != 0)
        }
        
        fn new() -> Self {
            TestStruct {
                inner: once_cell::unsync::OnceCell::new(),
            }
        }
        
        pub fn get(&self) -> Option<bool> {
            self.inner.get().map(Self::from_usize)
        }
    }

    let test_struct = TestStruct::new();
    assert_eq!(test_struct.get(), None);
}

