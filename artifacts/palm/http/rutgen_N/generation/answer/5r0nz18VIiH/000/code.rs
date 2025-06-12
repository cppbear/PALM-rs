// Answer 0

#[test]
fn test_fmt() {
    struct TestStruct {
        value: &'static str,
    }

    impl TestStruct {
        fn as_str(&self) -> &str {
            self.value
        }
    }

    let instance = TestStruct { value: "test" };
    let mut output = String::new();
    let result = std::fmt::Write::write_fmt(&mut output, format_args!("{:?}", instance.as_str()));

    assert!(result.is_ok());
    assert_eq!(output, "\"test\"");
}

