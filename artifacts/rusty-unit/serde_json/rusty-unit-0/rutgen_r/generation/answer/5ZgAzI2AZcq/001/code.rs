// Answer 0

#[test]
fn test_serialize_i16_begin_string_error() {
    struct MockWriter {
        should_fail: bool,
    }

    impl MockWriter {
        fn new(should_fail: bool) -> Self {
            MockWriter { should_fail }
        }
    }

    struct MockFormatter {
        writer: MockWriter,
    }

    impl MockFormatter {
        fn begin_string(&mut self, _: &mut MockWriter) -> Result<(), &'static str> {
            if self.writer.should_fail {
                Err("begin_string failed")
            } else {
                Ok(())
            }
        }

        fn write_i16(&mut self, _: &mut MockWriter, _: i16) -> Result<(), &'static str> {
            Ok(())
        }

        fn end_string(&mut self, _: &mut MockWriter) -> Result<(), &'static str> {
            Ok(())
        }
    }

    struct Serializer {
        formatter: MockFormatter,
        writer: MockWriter,
    }

    impl Serializer {
        fn serialize_i16(self, value: i16) -> Result<(), &'static str> {
            self.formatter.begin_string(&mut self.writer)?;
            self.formatter.write_i16(&mut self.writer, value)?;
            self.formatter.end_string(&mut self.writer)
        }
    }

    let mock_writer = MockWriter::new(true); // Simulates failure
    let serializer = Serializer {
        formatter: MockFormatter { writer: mock_writer },
        writer: MockWriter::new(false),
    };

    let result = serializer.serialize_i16(42);
    assert!(result.is_err());
    assert_eq!(result.err(), Some("begin_string failed"));
}

