// Answer 0

#[test]
fn test_lower_with_valid_start() {
    struct TestStruct {
        start: u8,
    }

    impl TestStruct {
        fn lower(&self) -> u8 {
            self.start
        }
    }

    let valid_input = TestStruct { start: 100 };
    assert_eq!(valid_input.lower(), 100);
}

#[test]
fn test_lower_with_minimum_start() {
    struct TestStruct {
        start: u8,
    }

    impl TestStruct {
        fn lower(&self) -> u8 {
            self.start
        }
    }

    let minimum_input = TestStruct { start: 0 };
    assert_eq!(minimum_input.lower(), 0);
}

#[test]
fn test_lower_with_maximum_start() {
    struct TestStruct {
        start: u8,
    }

    impl TestStruct {
        fn lower(&self) -> u8 {
            self.start
        }
    }

    let maximum_input = TestStruct { start: 255 };
    assert_eq!(maximum_input.lower(), 255);
}

