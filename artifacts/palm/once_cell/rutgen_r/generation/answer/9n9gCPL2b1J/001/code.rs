// Answer 0

#[test]
fn test_get_with_some_value() {
    use std::sync::AtomicPtr;
    use std::sync::Arc;
    use std::ptr;

    struct TestStruct {
        inner: AtomicPtr<i32>,
    }

    impl TestStruct {
        pub fn new(value: i32) -> Self {
            let boxed = Box::new(value);
            let ptr = Box::into_raw(boxed);
            TestStruct {
                inner: AtomicPtr::new(ptr),
            }
        }

        pub fn get(&self) -> Option<&i32> {
            let ptr = self.inner.load(std::sync::atomic::Ordering::Acquire);
            unsafe { ptr.as_ref() }
        }
    }

    let test_instance = TestStruct::new(42);
    assert_eq!(test_instance.get(), Some(&42));
}

#[test]
fn test_get_with_none_value() {
    use std::sync::AtomicPtr;
    use std::ptr;

    struct TestStruct {
        inner: AtomicPtr<i32>,
    }

    impl TestStruct {
        pub fn new() -> Self {
            TestStruct {
                inner: AtomicPtr::new(ptr::null_mut()),
            }
        }

        pub fn get(&self) -> Option<&i32> {
            let ptr = self.inner.load(std::sync::atomic::Ordering::Acquire);
            unsafe { ptr.as_ref() }
        }
    }

    let test_instance = TestStruct::new();
    assert_eq!(test_instance.get(), None);
}

