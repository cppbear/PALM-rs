// Answer 0

#[test]
fn test_fmt_debug() {
    struct TestStruct {
        value: &'static str,
    }

    impl TestStruct {
        fn as_str(&self) -> &str {
            self.value
        }
    }

    let instance = TestStruct { value: "http" };
    let mut output = Vec::new();
    let result = write!(&mut output, "{:?}", instance);

    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "\"http\"");
}

