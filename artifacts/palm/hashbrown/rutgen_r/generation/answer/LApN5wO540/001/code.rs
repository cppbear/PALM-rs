// Answer 0

#[test]
fn test_as_non_null() {
    use std::ptr::NonNull;

    struct TestStruct {
        data: Vec<i32>,
    }

    impl TestStruct {
        pub fn as_ptr(&self) -> *mut i32 {
            self.data.as_ptr() as *mut i32
        }

        pub fn as_non_null(&self) -> NonNull<i32> {
            unsafe { NonNull::new_unchecked(self.as_ptr()) }
        }
    }

    // Test with a non-empty vector, should succeed.
    let test_instance = TestStruct { data: vec![1, 2, 3] };
    let non_null_ptr = test_instance.as_non_null();
    assert!(!non_null_ptr.as_ptr().is_null());

    // Test with singleton vector, should succeed.
    let singleton_instance = TestStruct { data: vec![42] };
    let singleton_non_null = singleton_instance.as_non_null();
    assert!(!singleton_non_null.as_ptr().is_null());

    // Test with an empty vector, which should NOT panic but rather return a null pointer.
    let empty_instance = TestStruct { data: vec![] };
    let empty_non_null = empty_instance.as_ptr();
    assert!(empty_non_null.is_null());
}

