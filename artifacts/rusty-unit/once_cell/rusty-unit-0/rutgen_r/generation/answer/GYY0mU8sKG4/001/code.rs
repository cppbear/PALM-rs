// Answer 0

#[test]
fn test_get_some_value() {
    struct TestStruct {
        inner: once_cell::sync::Lazy<std::cell::Cell<Option<usize>>>,
    }

    impl TestStruct {
        fn from_usize(value: usize) -> Option<bool> {
            Some(value != 0)
        }

        fn new() -> Self {
            Self {
                inner: once_cell::sync::Lazy::new(|| std::cell::Cell::new(Some(1))),
            }
        }

        fn get(&self) -> Option<bool> {
            self.inner.get().map(Self::from_usize)
        }
    }

    let test_instance = TestStruct::new();
    assert_eq!(test_instance.get(), Some(true));
}

#[test]
fn test_get_none_value() {
    struct TestStruct {
        inner: once_cell::sync::Lazy<std::cell::Cell<Option<usize>>>,
    }

    impl TestStruct {
        fn from_usize(value: usize) -> Option<bool> {
            Some(value != 0)
        }

        fn new() -> Self {
            Self {
                inner: once_cell::sync::Lazy::new(|| std::cell::Cell::new(None)),
            }
        }

        fn get(&self) -> Option<bool> {
            self.inner.get().map(Self::from_usize)
        }
    }

    let test_instance = TestStruct::new();
    assert_eq!(test_instance.get(), None);
}

#[test]
fn test_get_zero_value() {
    struct TestStruct {
        inner: once_cell::sync::Lazy<std::cell::Cell<Option<usize>>>,
    }

    impl TestStruct {
        fn from_usize(value: usize) -> Option<bool> {
            Some(value != 0)
        }

        fn new() -> Self {
            Self {
                inner: once_cell::sync::Lazy::new(|| std::cell::Cell::new(Some(0))),
            }
        }

        fn get(&self) -> Option<bool> {
            self.inner.get().map(Self::from_usize)
        }
    }
    
    let test_instance = TestStruct::new();
    assert_eq!(test_instance.get(), Some(false));
}

