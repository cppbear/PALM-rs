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
fn test_expecting_valid_enum_name() {
    let test_enum = TestEnum { enum_name: "TestEnum" };
    let mut output = String::new();
    {
        let mut formatter = std::fmt::Formatter::new(&mut output);
        let result = test_enum.expecting(&mut formatter);
        assert!(result.is_ok());
    }
    assert_eq!(output, "variant of enum TestEnum");
}

#[test]
fn test_expecting_empty_enum_name() {
    let test_enum = TestEnum { enum_name: "" };
    let mut output = String::new();
    {
        let mut formatter = std::fmt::Formatter::new(&mut output);
        let result = test_enum.expecting(&mut formatter);
        assert!(result.is_ok());
    }
    assert_eq!(output, "variant of enum ");
}

#[test]
fn test_expecting_long_enum_name() {
    let test_enum = TestEnum { enum_name: "ThisIsAReallyLongEnumNameThatStretchesFar" };
    let mut output = String::new();
    {
        let mut formatter = std::fmt::Formatter::new(&mut output);
        let result = test_enum.expecting(&mut formatter);
        assert!(result.is_ok());
    }
    assert_eq!(output, "variant of enum ThisIsAReallyLongEnumNameThatStretchesFar");
}

