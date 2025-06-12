// Answer 0

#[test]
fn test_with_mut() {
    struct TestStruct {
        value: i32,
    }

    impl TestStruct {
        fn get_mut(&mut self) -> &mut *mut i32 {
            let ptr = &mut self.value as *mut i32;
            &mut ptr
        }

        fn with_mut<F, R>(&mut self, f: F) -> R
        where
            F: FnOnce(&mut *mut i32) -> R,
        {
            f(self.get_mut())
        }
    }

    let mut test_instance = TestStruct { value: 10 };

    let result = test_instance.with_mut(|ptr| {
        **ptr += 5;
        **ptr
    });
    
    assert_eq!(result, 15);
    assert_eq!(test_instance.value, 15);
}

