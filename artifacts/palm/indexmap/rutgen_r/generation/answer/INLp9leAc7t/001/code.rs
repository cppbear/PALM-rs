// Answer 0

#[test]
fn test_try_reserve_success() {
    struct Core {
        capacity: usize,
        len: usize,
    }

    impl Core {
        fn try_reserve(&mut self, additional: usize) -> Result<(), &'static str> {
            if self.len + additional > self.capacity {
                return Err("Exceeded capacity");
            }
            self.len += additional;
            Ok(())
        }
    }

    struct TestStruct {
        core: Core,
    }

    impl TestStruct {
        fn try_reserve(&mut self, additional: usize) -> Result<(), &'static str> {
            self.core.try_reserve(additional)
        }
    }

    let mut test_struct = TestStruct {
        core: Core {
            capacity: 10,
            len: 0,
        },
    };

    assert_eq!(test_struct.try_reserve(5), Ok(()));
    assert_eq!(test_struct.try_reserve(3), Ok(()));
}

#[test]
#[should_panic(expected = "Exceeded capacity")]
fn test_try_reserve_exceed_capacity() {
    struct Core {
        capacity: usize,
        len: usize,
    }

    impl Core {
        fn try_reserve(&mut self, additional: usize) -> Result<(), &'static str> {
            if self.len + additional > self.capacity {
                panic!("Exceeded capacity");
            }
            self.len += additional;
            Ok(())
        }
    }

    struct TestStruct {
        core: Core,
    }

    impl TestStruct {
        fn try_reserve(&mut self, additional: usize) -> Result<(), &'static str> {
            self.core.try_reserve(additional)
        }
    }

    let mut test_struct = TestStruct {
        core: Core {
            capacity: 5,
            len: 5,
        },
    };

    let _ = test_struct.try_reserve(1); // This should panic
}

