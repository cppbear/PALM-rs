// Answer 0

fn serialize_u64_test() -> Result<()> {
    struct MockWriter {
        error_on_begin: bool,
        error_on_write: bool,
    }

    impl MockWriter {
        fn new(error_on_begin: bool, error_on_write: bool) -> Self {
            MockWriter {
                error_on_begin,
                error_on_write,
            }
        }
    }

    struct MockFormatter {
        writer: MockWriter,
    }

    impl MockFormatter {
        fn begin_string(&mut self, _writer: &mut MockWriter) -> Result<()> {
            if self.writer.error_on_begin {
                Err(Error::io)
            } else {
                Ok(())
            }
        }

        fn write_u64(&mut self, _writer: &mut MockWriter, _value: u64) -> Result<()> {
            if self.writer.error_on_write {
                Err(Error::io)
            } else {
                Ok(())
            }
        }

        fn end_string(&mut self, _writer: &mut MockWriter) -> Result<()> {
            Ok(())
        }
    }

    struct MockSerializer {
        formatter: MockFormatter,
        writer: MockWriter,
    }

    impl MockSerializer {
        fn new(formatter: MockFormatter, writer: MockWriter) -> Self {
            MockSerializer { formatter, writer }
        }

        fn serialize_u64(self, value: u64) -> Result<()> {
            tri!(self.formatter.begin_string(&mut self.writer).map_err(Error::io));
            tri!(self.formatter.write_u64(&mut self.writer, value).map_err(Error::io));
            self.formatter.end_string(&mut self.writer).map_err(Error::io)
        }
    }

    #[test]
    fn test_serialize_u64_begin_error() {
        let writer = MockWriter::new(true, false);
        let formatter = MockFormatter { writer };
        let serializer = MockSerializer::new(formatter, writer);

        let result = serializer.serialize_u64(42);
        assert!(result.is_err());
    }

    #[test]
    fn test_serialize_u64_write_error() {
        let writer = MockWriter::new(false, true);
        let formatter = MockFormatter { writer };
        let serializer = MockSerializer::new(formatter, writer);

        let result = serializer.serialize_u64(42);
        assert!(result.is_err());
    }

    #[test]
    fn test_serialize_u64_success() {
        let writer = MockWriter::new(false, false);
        let formatter = MockFormatter { writer };
        let serializer = MockSerializer::new(formatter, writer);

        let result = serializer.serialize_u64(42);
        assert!(result.is_ok());
    }

    Ok(())
}

