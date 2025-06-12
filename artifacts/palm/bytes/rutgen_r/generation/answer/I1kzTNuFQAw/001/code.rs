// Answer 0

#[test]
fn test_with_mut_valid() {
    struct TestStruct {
        data: i32,
    }

    impl TestStruct {
        fn get_mut(&mut self) -> &mut *mut i32 {
            &mut self.data as *mut i32
        }
    }

    let mut test_instance = TestStruct { data: 10 };

    let result = test_instance.with_mut(|ptr| {
        **ptr += 5;
        *ptr
    });

    assert_eq!(unsafe { *result }, 15);
}

#[test]
#[should_panic]
fn test_with_mut_null_pointer() {
    struct TestStruct;

    impl TestStruct {
        fn get_mut(&mut self) -> &mut *mut i32 {
            &mut std::ptr::null_mut()
        }
    }

    let mut test_instance = TestStruct;

    let _result = test_instance.with_mut(|ptr| {
        let _ = unsafe { *ptr }; // This should trigger a panic
    });
}

#[test]
fn test_with_mut_boundary_condition() {
    struct TestStruct {
        data: i32,
    }

    impl TestStruct {
        fn get_mut(&mut self) -> &mut *mut i32 {
            &mut self.data as *mut i32
        }
    }

    let mut test_instance = TestStruct { data: std::i32::MAX };

    let result = test_instance.with_mut(|ptr| {
        **ptr += 1; // This could overflow
        *ptr
    });

    assert_eq!(unsafe { *result }, std::i32::MAX + 1); // Verify the overflow handling
}

