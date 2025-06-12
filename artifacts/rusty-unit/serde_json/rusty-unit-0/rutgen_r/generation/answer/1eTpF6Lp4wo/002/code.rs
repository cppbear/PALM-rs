// Answer 0

#[test]
fn test_fmt_with_line_zero() {
    struct TestStruct {
        line: usize,
        column: usize,
        code: String,
    }

    impl std::fmt::Display for TestStruct {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "Error code: {}", self.code)
        }
    }

    let test_case = TestStruct {
        line: 0,
        column: 5,
        code: "ERR123".to_string(),
    };

    let mut output = String::new();
    let result = std::fmt::write(&mut output, |f| test_case.fmt(f));

    assert!(result.is_ok());
    assert_eq!(output, "Error code: ERR123");
}

