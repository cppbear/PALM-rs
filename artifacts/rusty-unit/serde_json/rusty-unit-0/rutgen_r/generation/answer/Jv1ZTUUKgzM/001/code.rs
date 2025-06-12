// Answer 0

#[test]
fn test_serialize_f32_normal_value() {
    struct MockFormatter;
    struct MockWriter;

    impl MockFormatter {
        fn write_f32(&self, _writer: &mut MockWriter, _value: f32) -> Result<(), std::io::Error> {
            Ok(())
        }

        fn write_null(&self, _writer: &mut MockWriter) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct Serializer {
        formatter: MockFormatter,
        writer: MockWriter,
    }

    impl Serializer {
        fn new(formatter: MockFormatter, writer: MockWriter) -> Self {
            Serializer { formatter, writer }
        }

        fn serialize_f32(self, value: f32) -> Result<(), std::io::Error> {
            match value.classify() {
                std::num::FpCategory::Nan | std::num::FpCategory::Infinite => {
                    self.formatter.write_null(&mut self.writer)
                }
                _ => self.formatter.write_f32(&mut self.writer, value),
            }
        }
    }

    let formatter = MockFormatter;
    let writer = MockWriter;
    let serializer = Serializer::new(formatter, writer);

    let result = serializer.serialize_f32(42.0);
    assert!(result.is_ok());
}

#[test]
fn test_serialize_f32_zero_value() {
    struct MockFormatter;
    struct MockWriter;

    impl MockFormatter {
        fn write_f32(&self, _writer: &mut MockWriter, _value: f32) -> Result<(), std::io::Error> {
            Ok(())
        }

        fn write_null(&self, _writer: &mut MockWriter) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct Serializer {
        formatter: MockFormatter,
        writer: MockWriter,
    }

    impl Serializer {
        fn new(formatter: MockFormatter, writer: MockWriter) -> Self {
            Serializer { formatter, writer }
        }

        fn serialize_f32(self, value: f32) -> Result<(), std::io::Error> {
            match value.classify() {
                std::num::FpCategory::Nan | std::num::FpCategory::Infinite => {
                    self.formatter.write_null(&mut self.writer)
                }
                _ => self.formatter.write_f32(&mut self.writer, value),
            }
        }
    }

    let formatter = MockFormatter;
    let writer = MockWriter;
    let serializer = Serializer::new(formatter, writer);

    let result = serializer.serialize_f32(0.0);
    assert!(result.is_ok());
}

#[test]
fn test_serialize_f32_negative_value() {
    struct MockFormatter;
    struct MockWriter;

    impl MockFormatter {
        fn write_f32(&self, _writer: &mut MockWriter, _value: f32) -> Result<(), std::io::Error> {
            Ok(())
        }

        fn write_null(&self, _writer: &mut MockWriter) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct Serializer {
        formatter: MockFormatter,
        writer: MockWriter,
    }

    impl Serializer {
        fn new(formatter: MockFormatter, writer: MockWriter) -> Self {
            Serializer { formatter, writer }
        }

        fn serialize_f32(self, value: f32) -> Result<(), std::io::Error> {
            match value.classify() {
                std::num::FpCategory::Nan | std::num::FpCategory::Infinite => {
                    self.formatter.write_null(&mut self.writer)
                }
                _ => self.formatter.write_f32(&mut self.writer, value),
            }
        }
    }

    let formatter = MockFormatter;
    let writer = MockWriter;
    let serializer = Serializer::new(formatter, writer);

    let result = serializer.serialize_f32(-3.14);
    assert!(result.is_ok());
}

