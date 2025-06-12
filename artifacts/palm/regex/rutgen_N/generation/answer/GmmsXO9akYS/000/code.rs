// Answer 0

#[test]
fn test_finish_success() {
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

#[test]
#[should_panic]
fn test_finish_panics() {
    struct PanicStruct;

    impl PanicStruct {
        fn finish(self) -> std::fmt::Result {
            panic!("Intentional panic");
        }
    }

    let test_instance = PanicStruct;
    test_instance.finish();
}

