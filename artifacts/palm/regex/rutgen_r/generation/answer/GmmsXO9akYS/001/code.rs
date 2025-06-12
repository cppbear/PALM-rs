// Answer 0

#[test]
fn test_finish() {
    struct TestStruct;

    impl TestStruct {
        fn finish(self) -> std::fmt::Result {
            Ok(())
        }
    }

    let test_instance = TestStruct;
    let result = test_instance.finish();
    assert!(result.is_ok());
}

