// Answer 0

#[test]
fn test_collect_str_with_valid_string() {
    struct MockSer {
        collected: String,
    }

    impl MockSer {
        fn collect_str<T>(&mut self, value: &T) -> Result<(), String>
        where
            T: ?Sized + std::fmt::Display,
        {
            self.collected.push_str(&value.to_string());
            Ok(())
        }
    }

    struct TestStruct {
        ser: MockSer,
    }

    impl TestStruct {
        fn new() -> Self {
            TestStruct {
                ser: MockSer {
                    collected: String::new(),
                },
            }
        }

        fn collect_str<T>(&mut self, value: &T) -> Result<(), String>
        where
            T: ?Sized + std::fmt::Display,
        {
            self.ser.collect_str(value)
        }
    }

    let mut test = TestStruct::new();
    let result = test.collect_str(&"Hello, World!");
    assert!(result.is_ok());
}

#[test]
fn test_collect_str_with_empty_string() {
    struct MockSer {
        collected: String,
    }

    impl MockSer {
        fn collect_str<T>(&mut self, value: &T) -> Result<(), String>
        where
            T: ?Sized + std::fmt::Display,
        {
            self.collected.push_str(&value.to_string());
            Ok(())
        }
    }

    struct TestStruct {
        ser: MockSer,
    }

    impl TestStruct {
        fn new() -> Self {
            TestStruct {
                ser: MockSer {
                    collected: String::new(),
                },
            }
        }

        fn collect_str<T>(&mut self, value: &T) -> Result<(), String>
        where
            T: ?Sized + std::fmt::Display,
        {
            self.ser.collect_str(value)
        }
    }

    let mut test = TestStruct::new();
    let result = test.collect_str(&"");
    assert!(result.is_ok());
}

#[test]
fn test_collect_str_with_string_slice() {
    struct MockSer {
        collected: String,
    }

    impl MockSer {
        fn collect_str<T>(&mut self, value: &T) -> Result<(), String>
        where
            T: ?Sized + std::fmt::Display,
        {
            self.collected.push_str(&value.to_string());
            Ok(())
        }
    }

    struct TestStruct {
        ser: MockSer,
    }

    impl TestStruct {
        fn new() -> Self {
            TestStruct {
                ser: MockSer {
                    collected: String::new(),
                },
            }
        }

        fn collect_str<T>(&mut self, value: &T) -> Result<(), String>
        where
            T: ?Sized + std::fmt::Display,
        {
            self.ser.collect_str(value)
        }
    }

    let mut test = TestStruct::new();
    let string_slice: &str = "String Slice Test";
    let result = test.collect_str(&string_slice);
    assert!(result.is_ok());
}

