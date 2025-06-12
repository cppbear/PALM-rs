// Answer 0

#[cfg(test)]
mod tests {
    use std::fmt;

    struct TestStruct<'a> {
        type_name: &'a str,
        variant_name: &'a str,
    }

    impl<'a> TestStruct<'a> {
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(
                formatter,
                "unit variant {}::{}",
                self.type_name, self.variant_name
            )
        }
    }

    #[test]
    fn test_expectation_valid() {
        let instance = TestStruct {
            type_name: "TypeName",
            variant_name: "VariantName",
        };
        let mut output = String::new();
        let result = instance.expecting(&mut fmt::Formatter::new(&mut output));

        assert!(result.is_ok());
        assert_eq!(output, "unit variant TypeName::VariantName");
    }

    #[test]
    #[should_panic]
    fn test_expectation_empty_names() {
        let instance = TestStruct {
            type_name: "",
            variant_name: "",
        };
        let mut output = String::new();
        let _ = instance.expecting(&mut fmt::Formatter::new(&mut output)); // This should panic
    }

    #[test]
    fn test_expectation_long_names() {
        let instance = TestStruct {
            type_name: "AReallyLongTypeNameThatExceedsNormalLength",
            variant_name: "AnotherReallyLongVariantNameThatExceedsNormalLength",
        };
        let mut output = String::new();
        let result = instance.expecting(&mut fmt::Formatter::new(&mut output));

        assert!(result.is_ok());
        assert_eq!(output, "unit variant AReallyLongTypeNameThatExceedsNormalLength::AnotherReallyLongVariantNameThatExceedsNormalLength");
    }
}

