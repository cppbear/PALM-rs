// Answer 0

#[test]
fn test_end() {
    struct TestStruct {
        void: !,
    }

    impl TestStruct {
        fn new() -> Self {
            // The 'void' field of type '!' cannot be instantiated, 
            // so we don't need to actually create an instance.
            TestStruct { void: unreachable!() }
        }

        fn end(self) -> Result<(), &'static str> {
            match self.void {}
        }
    }

    let test_instance = TestStruct::new();
    let result = panic::catch_unwind(|| {
        test_instance.end()
    });
    assert!(result.is_err());
}

