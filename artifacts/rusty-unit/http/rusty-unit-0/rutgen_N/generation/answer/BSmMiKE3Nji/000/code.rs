// Answer 0

#[cfg(test)]
mod tests {
    use std::any::Any;

    struct TestStruct;

    impl TestStruct {
        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    #[test]
    fn test_as_any() {
        let test_instance = TestStruct;
        let result: &dyn Any = test_instance.as_any();
        assert!(result.is::<TestStruct>());
    }
}

