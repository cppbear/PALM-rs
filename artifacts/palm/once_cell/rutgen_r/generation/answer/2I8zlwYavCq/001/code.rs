// Answer 0

#[test]
fn test_compare_exchange_success() {
    use std::sync::atomic::{AtomicPtr, Ordering};
    use std::ptr;

    struct TestStruct {
        inner: AtomicPtr<i32>,
    }

    impl TestStruct {
        fn new() -> Self {
            TestStruct {
                inner: AtomicPtr::new(ptr::null_mut()),
            }
        }

        fn compare_exchange(&self, value: &'static i32) -> Result<(), *const i32> {
            self.inner
                .compare_exchange(
                    ptr::null_mut(),
                    value as *const _ as *mut _,
                    Ordering::Release,
                    Ordering::Acquire,
                )
                .map(|_: *mut i32| ())
                .map_err(|v| v as *const i32)
        }
    }

    let test_instance = TestStruct::new();
    let test_value = Box::new(42);
    let result = test_instance.compare_exchange(&test_value);
    
    assert_eq!(result.is_ok(), true);
}

#[test]
fn test_compare_exchange_failure() {
    use std::sync::atomic::{AtomicPtr, Ordering};
    use std::ptr;

    struct TestStruct {
        inner: AtomicPtr<i32>,
    }

    impl TestStruct {
        fn new() -> Self {
            TestStruct {
                inner: AtomicPtr::new(ptr::null_mut()),
            }
        }

        fn compare_exchange(&self, value: &'static i32) -> Result<(), *const i32> {
            self.inner
                .compare_exchange(
                    ptr::null_mut(),
                    value as *const _ as *mut _,
                    Ordering::Release,
                    Ordering::Acquire,
                )
                .map(|_: *mut i32| ())
                .map_err(|v| v as *const i32)
        }
    }

    let value1 = Box::new(42);
    let value2 = Box::new(43);
    let test_instance = TestStruct::new();

    // First comparison with the null pointer should succeed
    let result1 = test_instance.compare_exchange(&value1);
    assert_eq!(result1.is_ok(), true);
    
    // Now the inner value is set, next call with a new value should fail
    let result2 = test_instance.compare_exchange(&value2);
    assert_eq!(result2.is_err(), true);
}

#[test]
#[should_panic]
fn test_compare_exchange_null_pointer() {
    use std::sync::atomic::{AtomicPtr, Ordering};
    use std::ptr;

    struct TestStruct {
        inner: AtomicPtr<i32>,
    }

    impl TestStruct {
        fn new() -> Self {
            TestStruct {
                inner: AtomicPtr::new(ptr::null_mut()),
            }
        }

        fn compare_exchange(&self, value: &'static i32) -> Result<(), *const i32> {
            self.inner
                .compare_exchange(
                    ptr::null_mut(),
                    value as *const _ as *mut _,
                    Ordering::Release,
                    Ordering::Acquire,
                )
                .map(|_: *mut i32| ())
                .map_err(|v| v as *const i32)
        }
    }

    let test_instance = TestStruct::new();
    // Attempting to compare with a reference to a potentially dangling pointer
    let null_value: &'static i32 = unsafe { &*(ptr::null_mut()) };
    let _ = test_instance.compare_exchange(null_value);
}

