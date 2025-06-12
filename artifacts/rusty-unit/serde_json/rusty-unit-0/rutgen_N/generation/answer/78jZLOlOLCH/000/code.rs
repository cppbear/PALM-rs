// Answer 0

#[test]
fn test_collect_str_valid() {
    struct TestSerializer {
        collected: String,
    }

    impl TestSerializer {
        fn new() -> Self {
            Self { collected: String::new() }
        }

        fn collect_str(&mut self, value: &impl std::fmt::Display) -> Result<(), std::fmt::Error> {
            self.collected.push_str(&value.to_string());
            Ok(())
        }
    }

    fn collect_str<T>(ser: &mut T, value: &impl std::fmt::Display) -> Result<(), std::fmt::Error> 
    where 
        T: ?Sized + TestSerializer + std::fmt::Write {
        ser.collect_str(value)
    }

    let mut serializer = TestSerializer::new();
    let result = collect_str(&mut serializer, &"Test string");
    assert!(result.is_ok());
    assert_eq!(serializer.collected, "Test string");
}

#[test]
#[should_panic]
fn test_collect_str_invalid() {
    struct TestSerializer {
        collected: String,
    }

    impl TestSerializer {
        fn new() -> Self {
            Self { collected: String::new() }
        }

        fn collect_str(&mut self, _: &impl std::fmt::Display) -> Result<(), std::fmt::Error> {
            // Simulating a panic condition
            panic!("Simulated panic");
        }
    }

    fn collect_str<T>(ser: &mut T, value: &impl std::fmt::Display) -> Result<(), std::fmt::Error> 
    where 
        T: ?Sized + TestSerializer {
        ser.collect_str(value)
    }

    let mut serializer = TestSerializer::new();
    let _ = collect_str(&mut serializer, &"Should panic");
}

