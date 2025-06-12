// Answer 0

#[test]
fn test_fmt_success() {
    struct DummyStruct;

    impl std::fmt::Display for DummyStruct {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "DummyStruct") // Sample implementation for testing
        }
    }

    let dummy = DummyStruct;
    let mut output = String::new();
    {
        let mut formatter = std::fmt::Formatter::new(&mut output);
        let result = dummy.fmt(&mut formatter);
        assert!(result.is_ok());
    }
    assert_eq!(output, "DummyStruct");
}

