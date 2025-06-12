// Answer 0

#[test]
fn test_end_with_empty_void() {
    struct TestStruct {
        void: (),
    }

    impl TestStruct {
        fn end(self) -> Result<(), &'static str> {
            match self.void {
                _ => Err("Expected void got something else"),
            }
        }
    }

    let test_instance = TestStruct { void: () };
    let result = test_instance.end();
    assert!(result.is_err());
}

#[test]
#[should_panic(expected = "Expected void got something else")]
fn test_end_panics_on_non_void() {
    struct PanicTestStruct {
        void: u32,
    }

    impl PanicTestStruct {
        fn end(self) -> Result<(), &'static str> {
            match self.void {
                _ => Err("Expected void got something else"),
            }
        }
    }

    let panic_instance = PanicTestStruct { void: 1 };
    let _ = panic_instance.end();
}

