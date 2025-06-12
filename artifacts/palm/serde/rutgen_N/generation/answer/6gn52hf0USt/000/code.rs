// Answer 0

#[derive(Debug)]
struct TestEnum {
    enum_name: &'static str,
}

impl TestEnum {
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "variant of enum {}", self.enum_name)
    }
}

#[test]
fn test_expecting() {
    let enum_instance = TestEnum { enum_name: "TestEnumVariant" };
    let mut output = String::new();
    let result = enum_instance.expecting(&mut output);

    assert!(result.is_ok());
    assert_eq!(output, "variant of enum TestEnumVariant");
}

#[test]
fn test_expecting_empty_enum_name() {
    let enum_instance = TestEnum { enum_name: "" };
    let mut output = String::new();
    let result = enum_instance.expecting(&mut output);

    assert!(result.is_ok());
    assert_eq!(output, "variant of enum ");
}

#[test]
fn test_expecting_long_enum_name() {
    let enum_instance = TestEnum { enum_name: "ThisIsAVeryLongEnumNameForTestingPurposes" };
    let mut output = String::new();
    let result = enum_instance.expecting(&mut output);

    assert!(result.is_ok());
    assert_eq!(output, "variant of enum ThisIsAVeryLongEnumNameForTestingPurposes");
}

