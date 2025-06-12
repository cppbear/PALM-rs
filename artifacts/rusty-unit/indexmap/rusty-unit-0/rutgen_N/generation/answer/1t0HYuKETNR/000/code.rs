// Answer 0

#[test]
fn test_try_reserve_exact_success() {
    struct TestStruct {
        map: std::collections::HashMap<i32, i32>,
    }

    impl TestStruct {
        fn new() -> Self {
            TestStruct {
                map: std::collections::HashMap::new(),
            }
        }

        fn try_reserve_exact(&mut self, additional: usize) -> Result<(), &'static str> {
            if self.map.len() + additional > 100 {
                return Err("Exceeds maximum capacity");
            }
            self.map.reserve(additional);
            Ok(())
        }
    }

    let mut test_struct = TestStruct::new();
    
    let result = test_struct.try_reserve_exact(10);
    assert!(result.is_ok());
}

#[test]
#[should_panic(expected = "Exceeds maximum capacity")]
fn test_try_reserve_exact_exceed_capacity() {
    struct TestStruct {
        map: std::collections::HashMap<i32, i32>,
    }

    impl TestStruct {
        fn new() -> Self {
            TestStruct {
                map: std::collections::HashMap::new(),
            }
        }

        fn try_reserve_exact(&mut self, additional: usize) -> Result<(), &'static str> {
            if self.map.len() + additional > 100 {
                panic!("Exceeds maximum capacity");
            }
            self.map.reserve(additional);
            Ok(())
        }
    }

    let mut test_struct = TestStruct::new();
    test_struct.try_reserve_exact(101).unwrap();
}

