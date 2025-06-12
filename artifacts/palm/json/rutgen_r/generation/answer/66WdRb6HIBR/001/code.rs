// Answer 0

fn test_serialize_bool_begin_string_error() -> Result<()> {
    struct MockWriter;

    impl MockWriter {
        fn begin_string(&mut self) -> Result<()> {
            Err(Error::io(io::Error::new(io::ErrorKind::Other, "begin_string error")))
        }

        fn write_bool(&mut self, _value: bool) -> Result<()> {
            Ok(())
        }

        fn end_string(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter {
        writer: MockWriter,
    }

    impl MockFormatter {
        fn begin_string(&mut self, _writer: &mut MockWriter) -> Result<()> {
            self.writer.begin_string()
        }

        fn write_bool(&mut self, _writer: &mut MockWriter, _value: bool) -> Result<()> {
            self.writer.write_bool(true)
        }

        fn end_string(&mut self, _writer: &mut MockWriter) -> Result<()> {
            self.writer.end_string()
        }
    }

    struct MockSerializer {
        formatter: MockFormatter,
    }

    let mock_writer = MockWriter;
    let mock_formatter = MockFormatter { writer: mock_writer };
    let mut serializer = MockSerializer { formatter: mock_formatter };

    let result = serializer.serialize_bool(true);
    assert!(result.is_err());

    Ok(())
}

