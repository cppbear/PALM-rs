// Answer 0

#[test]
fn test_expecting() {
    use std::fmt;

    struct MockFormatter {
        output: String,
    }

    impl fmt::Write for MockFormatter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct TestStruct {
        type_name: &'static str,
        variant_name: &'static str,
    }

    impl TestStruct {
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(
                formatter,
                "unit variant {}::{}",
                self.type_name, self.variant_name
            )
        }
    }

    // Test case 1: Normal case with simple strings
    let test_obj = TestStruct {
        type_name: "ExampleType",
        variant_name: "ExampleVariant",
    };
    let mut formatter = MockFormatter { output: String::new() };
    test_obj.expecting(&mut formatter).unwrap();
    assert_eq!(formatter.output, "unit variant ExampleType::ExampleVariant");

    // Test case 2: Test with empty strings
    let test_obj_empty = TestStruct {
        type_name: "",
        variant_name: "",
    };
    let mut formatter_empty = MockFormatter { output: String::new() };
    test_obj_empty.expecting(&mut formatter_empty).unwrap();
    assert_eq!(formatter_empty.output, "unit variant ::");

    // Test case 3: Test with long strings
    let test_obj_long = TestStruct {
        type_name: "LongTypeNameThatExceedsNormalLength",
        variant_name: "VeryLongVariantNameThatMightStressTest",
    };
    let mut formatter_long = MockFormatter { output: String::new() };
    test_obj_long.expecting(&mut formatter_long).unwrap();
    assert_eq!(formatter_long.output, "unit variant LongTypeNameThatExceedsNormalLength::VeryLongVariantNameThatMightStressTest");
}

