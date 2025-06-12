// Answer 0

#[test]
fn test_as_any() {
    struct TestStruct;

    impl AnyClone for TestStruct {
        fn clone_box(&self) -> Box<dyn AnyClone + Send + Sync> {
            Box::new(TestStruct)
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

    let test_instance = TestStruct;
    let any_ref: &dyn Any = test_instance.as_any();

    // Assert that any_ref can be downcast to TestStruct
    assert!(any_ref.is::<TestStruct>());
}

#[test]
fn test_clone_box() {
    struct TestCloneStruct;

    impl AnyClone for TestCloneStruct {
        fn clone_box(&self) -> Box<dyn AnyClone + Send + Sync> {
            Box::new(TestCloneStruct)
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

    let original = TestCloneStruct;
    let cloned: Box<dyn AnyClone + Send + Sync> = original.clone_box();
    
    // Assert that the cloned instance is a TestCloneStruct
    assert!(cloned.as_any().is::<TestCloneStruct>());
}

