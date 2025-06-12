// Answer 0

#[test]
fn test_as_any_mut() {
    struct TestStruct {
        value: i32,
    }

    impl TestStruct {
        fn new(value: i32) -> Self {
            TestStruct { value }
        }
    }

    let mut test_instance = TestStruct::new(42);
    let any_ref: &mut dyn Any = test_instance.as_any_mut();
    let downcasted_ref = any_ref.downcast_mut::<TestStruct>().unwrap();

    assert_eq!(downcasted_ref.value, 42);
}

#[test]
fn test_as_any_mut_with_modified_value() {
    struct TestStruct {
        value: i32,
    }

    impl TestStruct {
        fn new(value: i32) -> Self {
            TestStruct { value }
        }
    }

    let mut test_instance = TestStruct::new(10);
    let any_ref: &mut dyn Any = test_instance.as_any_mut();
    let downcasted_ref = any_ref.downcast_mut::<TestStruct>().unwrap();
    
    downcasted_ref.value = 100;

    assert_eq!(test_instance.value, 100);
}

