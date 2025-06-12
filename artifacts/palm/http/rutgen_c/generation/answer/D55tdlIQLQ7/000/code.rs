// Answer 0

#[test]
fn test_into_any() {
    struct TestStruct;

    let test_instance = Box::new(TestStruct);
    let any_instance: Box<dyn Any> = test_instance.into_any();

    // Assert that the boxed instance can be downcast back to the original type
    let downcasted_instance = any_instance.downcast::<TestStruct>();
    assert!(downcasted_instance.is_ok());
}

#[test]
fn test_clone_box() {
    struct CloneableStruct {
        value: i32,
    }

    impl Clone for CloneableStruct {
        fn clone(&self) -> Self {
            CloneableStruct { value: self.value }
        }
    }

    impl AnyClone for CloneableStruct {
        fn clone_box(&self) -> Box<dyn AnyClone + Send + Sync> {
            Box::new(self.clone())
        }

        fn as_any(&self) -> &dyn Any {
            self
        }

        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }

        fn into_any(self: Box<Self>) -> Box<dyn Any> {
            self
        }
    }

    let original = CloneableStruct { value: 42 };
    let boxed: Box<dyn AnyClone + Send + Sync> = Box::new(original);
    let cloned_boxed = boxed.clone_box();

    let original_value = boxed.as_any().downcast_ref::<CloneableStruct>().unwrap().value;
    let cloned_value = cloned_boxed.as_any().downcast_ref::<CloneableStruct>().unwrap().value;

    assert_eq!(original_value, cloned_value);
}

