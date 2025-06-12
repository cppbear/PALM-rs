// Answer 0

#[test]
fn test_write_function() {
    use std::ptr;

    struct TestStruct {
        value: i32,
    }

    let mut data = TestStruct { value: 42 };
    let data_ptr: *mut TestStruct = &mut data;

    unsafe {
        // Overwrite the value at the memory location with a new value
        (*data_ptr).value = 100; // Set initial value
        ptr::write(data_ptr, TestStruct { value: 84 }); // Use the write function

        // Now the value should be 84
        assert_eq!((*data_ptr).value, 84);
    }
}

#[test]
#[should_panic]
fn test_write_with_invalid_hash_eq() {
    use std::ptr;

    #[derive(Debug)]
    struct InvalidTestStruct {
        value: u32,
    }

    impl PartialEq for InvalidTestStruct {
        fn eq(&self, other: &Self) -> bool {
            self.value % 2 == other.value % 2 // Only consider even/odd for equality
        }
    }

    impl Eq for InvalidTestStruct {}

    let mut data = InvalidTestStruct { value: 1 };
    let data_ptr: *mut InvalidTestStruct = &mut data;

    unsafe {
        ptr::write(data_ptr, InvalidTestStruct { value: 2 }); // Value changed, but hash/eq does not match

        // This will panic due to violation of Hash and Eq guarantees
        assert_eq!((*data_ptr).value, 2);
    }
}

