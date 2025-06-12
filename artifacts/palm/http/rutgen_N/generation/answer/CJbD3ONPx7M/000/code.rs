// Answer 0

#[test]
fn test_fmt_display() {
    struct TestStruct;

    impl std::fmt::Display for TestStruct {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "TestStruct Display")
        }
    }

    let test_instance = TestStruct;
    let result = format!("{}", test_instance);
    assert_eq!(result, "TestStruct Display");
}

