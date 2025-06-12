// Answer 0

#[test]
fn test_clone_box() {
    struct TestStruct {
        value: i32,
    }

    impl Clone for TestStruct {
        fn clone(&self) -> Self {
            TestStruct {
                value: self.value,
            }
        }
    }

    let original = TestStruct { value: 42 };
    let cloned: Box<dyn AnyClone + Send + Sync> = original.clone_box();

    let cloned_value = cloned.as_any().downcast_ref::<TestStruct>().unwrap();
    assert_eq!(cloned_value.value, 42);
}

#[test]
fn test_clone_box_multiple_clones() {
    struct TestStruct {
        value: i32,
    }

    impl Clone for TestStruct {
        fn clone(&self) -> Self {
            TestStruct {
                value: self.value,
            }
        }
    }

    let original = TestStruct { value: 100 };
    let cloned1: Box<dyn AnyClone + Send + Sync> = original.clone_box();
    let cloned2: Box<dyn AnyClone + Send + Sync> = cloned1.clone_box();

    let first_clone_value = cloned1.as_any().downcast_ref::<TestStruct>().unwrap();
    let second_clone_value = cloned2.as_any().downcast_ref::<TestStruct>().unwrap();
    
    assert_eq!(first_clone_value.value, 100);
    assert_eq!(second_clone_value.value, 100);
    assert_ne!(first_clone_value as *const _, second_clone_value as *const _);
}

