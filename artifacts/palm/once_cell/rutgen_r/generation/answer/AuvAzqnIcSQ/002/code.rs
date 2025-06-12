// Answer 0

#[test]
fn test_get_non_null_pointer() {
    use std::sync::atomic::{AtomicPtr, Ordering};

    struct TestStruct {
        value: usize,
    }

    struct TestInner {
        inner: AtomicPtr<TestStruct>,
    }

    impl TestInner {
        pub fn get(&self) -> Option<&TestStruct> {
            let ptr = self.inner.load(Ordering::Acquire);
            if ptr.is_null() {
                return None;
            }
            Some(unsafe { &*ptr })
        }
    }

    // Create a TestStruct and store its pointer in AtomicPtr
    let test_value = TestStruct { value: 42 };
    let ptr = Box::into_raw(Box::new(test_value));

    let test_inner = TestInner {
        inner: AtomicPtr::new(ptr),
    };

    // Test that get returns Some reference to TestStruct
    let result = test_inner.get();
    assert!(result.is_some());
    assert_eq!(result.unwrap().value, 42);

    // Clean up
    unsafe { Box::from_raw(ptr); } // Recovers the Box to safely free the memory
}

