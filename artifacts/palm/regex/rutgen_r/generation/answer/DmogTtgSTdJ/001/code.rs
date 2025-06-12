// Answer 0

#[test]
fn test_finish() {
    struct TestStruct;

    impl TestStruct {
        fn finish(self) -> std::fmt::Result {
            Ok(())
        }
    }

    let instance = TestStruct;
    let result = instance.finish();
    assert_eq!(result, Ok(()));
}

