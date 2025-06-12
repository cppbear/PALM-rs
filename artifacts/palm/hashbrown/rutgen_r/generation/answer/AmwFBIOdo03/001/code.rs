// Answer 0

#[test]
fn test_data_end_non_null_pointer() {
    use std::ptr::NonNull;

    struct RawTableInner<T> {
        ctrl: NonNull<T>,
    }

    impl RawTableInner<i32> {
        fn new(value: i32) -> Self {
            let ctrl = Box::new(value);
            let ctrl_ptr = NonNull::new(Box::into_raw(ctrl)).expect("Failed to create NonNull pointer");
            Self { ctrl: ctrl_ptr }
        }
    }

    let table = RawTableInner::new(42);
    let end_ptr: NonNull<i32> = table.data_end();
    assert_eq!(unsafe { *end_ptr.as_ptr() }, 42);
}

#[test]
#[should_panic]
fn test_data_end_invalid_usage() {
    use std::ptr::NonNull;

    struct RawTableInner<T> {
        ctrl: NonNull<T>,
    }

    impl RawTableInner<i32> {
        fn new() -> Self {
            let ctrl = Box::new(0);
            let ctrl_ptr = NonNull::new(Box::into_raw(ctrl)).expect("Failed to create NonNull pointer");
            Self { ctrl: ctrl_ptr }
        }

        fn drop_inner(self) {
            unsafe { Box::from_raw(self.ctrl.as_ptr()); } // Properly drop the Box to avoid memory leak
        }
    }

    let table = RawTableInner::new();
    table.drop_inner(); // Drop the inner Box to invalidate the pointer
    let _end_ptr: NonNull<i32> = table.data_end(); // This should panic due to use-after-free.
}

