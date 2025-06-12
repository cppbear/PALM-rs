// Answer 0

#[test]
fn test_serialize_f32_infinite() {
    struct MockFormatter {
        output: String,
    }

    impl MockFormatter {
        fn new() -> Self {
            MockFormatter { output: String::new() }
        }

        fn write_null(&mut self, _writer: &mut std::fmt::Write) -> Result<(), std::io::Error> {
            self.output.push_str("null");
            Ok(())
        }

        fn write_f32(&mut self, _writer: &mut std::fmt::Write, value: f32) -> Result<(), std::io::Error> {
            self.output.push_str(&value.to_string());
            Ok(())
        }
    }

    struct TestStruct<'a> {
        formatter: MockFormatter,
        writer: &'a mut String,
    }

    impl<'a> TestStruct<'a> {
        fn serialize_f32(self, value: f32) -> Result<(), std::io::Error> {
            match value.classify() {
                std::num::FpCategory::Nan | std::num::FpCategory::Infinite => self
                    .formatter
                    .write_null(self.writer)
                    .map_err(Error::io),
                _ => self
                    .formatter
                    .write_f32(self.writer, value)
                    .map_err(Error::io),
            }
        }
    }

    let mut output = String::new();
    let formatter = MockFormatter::new();
    let test_struct = TestStruct { formatter, writer: &mut output };

    // Testing with f32::INFINITY
    let result = test_struct.serialize_f32(f32::INFINITY);
    assert!(result.is_ok());
    assert_eq!(output, "null");
}

#[test]
fn test_serialize_f32_nan() {
    struct MockFormatter {
        output: String,
    }

    impl MockFormatter {
        fn new() -> Self {
            MockFormatter { output: String::new() }
        }

        fn write_null(&mut self, _writer: &mut std::fmt::Write) -> Result<(), std::io::Error> {
            self.output.push_str("null");
            Ok(())
        }

        fn write_f32(&mut self, _writer: &mut std::fmt::Write, value: f32) -> Result<(), std::io::Error> {
            self.output.push_str(&value.to_string());
            Ok(())
        }
    }

    struct TestStruct<'a> {
        formatter: MockFormatter,
        writer: &'a mut String,
    }

    impl<'a> TestStruct<'a> {
        fn serialize_f32(self, value: f32) -> Result<(), std::io::Error> {
            match value.classify() {
                std::num::FpCategory::Nan | std::num::FpCategory::Infinite => self
                    .formatter
                    .write_null(self.writer)
                    .map_err(Error::io),
                _ => self
                    .formatter
                    .write_f32(self.writer, value)
                    .map_err(Error::io),
            }
        }
    }

    let mut output = String::new();
    let formatter = MockFormatter::new();
    let test_struct = TestStruct { formatter, writer: &mut output };

    // Testing with f32::NAN
    let result = test_struct.serialize_f32(f32::NAN);
    assert!(result.is_ok());
    assert_eq!(output, "null");
}

