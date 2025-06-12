// Answer 0

#[cfg(test)]
mod tests {
    use std::any::Any;

    struct TestStruct;

    impl TestStruct {
        fn into_any(self: Box<Self>) -> Box<dyn Any> {
            self
        }
    }

    #[test]
    fn test_into_any() {
        let test_instance = Box::new(TestStruct);
        let any_instance: Box<dyn Any> = test_instance.into_any();
        assert!(any_instance.is::<TestStruct>());
    }
    
    #[test]
    fn test_into_any_null() {
        let test_instance: Box<TestStruct> = Box::new(TestStruct);
        let any_instance: Box<dyn Any> = test_instance.into_any();
        assert!(any_instance.is::<TestStruct>());
    }
}

