// Answer 0

#[test]
fn test_serialize_u128_begin_string_error() {
    struct MockWriter;

    impl std::io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "write error"))
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl MockFormatter {
        fn begin_string(&mut self, _writer: &mut dyn std::io::Write) -> std::io::Result<()> {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "begin string error"))
        }
        
        fn write_u128(&mut self, _writer: &mut dyn std::io::Write, _value: u128) -> std::io::Result<()> {
            Ok(())
        }

        fn end_string(&mut self, _writer: &mut dyn std::io::Write) -> std::io::Result<()> {
            Ok(())
        }
    }

    struct Serializer {
        writer: MockWriter,
        formatter: MockFormatter,
    }

    impl Serializer {
        fn new(writer: MockWriter, formatter: MockFormatter) -> Self {
            Serializer { writer, formatter }
        }

        fn serialize_u128(self, value: u128) -> Result<(), std::io::Error> {
            self.formatter.begin_string(&mut self.writer)?;
            self.formatter.write_u128(&mut self.writer, value)?;
            self.formatter.end_string(&mut self.writer)
        }
    }

    let serializer = Serializer::new(MockWriter, MockFormatter);
    let result = serializer.serialize_u128(42);
    assert!(result.is_err());
}

#[test]
fn test_serialize_u128_write_u128_error() {
    struct MockWriter;

    impl std::io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    struct MockFormatter {
        write_error: bool,
    }

    impl MockFormatter {
        fn new(write_error: bool) -> Self {
            MockFormatter { write_error }
        }

        fn begin_string(&mut self, _writer: &mut dyn std::io::Write) -> std::io::Result<()> {
            Ok(())
        }
        
        fn write_u128(&mut self, _writer: &mut dyn std::io::Write, _value: u128) -> std::io::Result<()> {
            if self.write_error {
                Err(std::io::Error::new(std::io::ErrorKind::Other, "write u128 error"))
            } else {
                Ok(())
            }
        }

        fn end_string(&mut self, _writer: &mut dyn std::io::Write) -> std::io::Result<()> {
            Ok(())
        }
    }

    struct Serializer {
        writer: MockWriter,
        formatter: MockFormatter,
    }

    impl Serializer {
        fn new(writer: MockWriter, formatter: MockFormatter) -> Self {
            Serializer { writer, formatter }
        }

        fn serialize_u128(self, value: u128) -> Result<(), std::io::Error> {
            self.formatter.begin_string(&mut self.writer)?;
            self.formatter.write_u128(&mut self.writer, value)?;
            self.formatter.end_string(&mut self.writer)
        }
    }

    let serializer = Serializer::new(MockWriter, MockFormatter::new(true));
    let result = serializer.serialize_u128(42);
    assert!(result.is_err());
}

#[test]
fn test_serialize_u128_end_string_error() {
    struct MockWriter;

    impl std::io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    struct MockFormatter {
        end_error: bool,
    }

    impl MockFormatter {
        fn new(end_error: bool) -> Self {
            MockFormatter { end_error }
        }

        fn begin_string(&mut self, _writer: &mut dyn std::io::Write) -> std::io::Result<()> {
            Ok(())
        }
        
        fn write_u128(&mut self, _writer: &mut dyn std::io::Write, _value: u128) -> std::io::Result<()> {
            Ok(())
        }

        fn end_string(&mut self, _writer: &mut dyn std::io::Write) -> std::io::Result<()> {
            if self.end_error {
                Err(std::io::Error::new(std::io::ErrorKind::Other, "end string error"))
            } else {
                Ok(())
            }
        }
    }

    struct Serializer {
        writer: MockWriter,
        formatter: MockFormatter,
    }

    impl Serializer {
        fn new(writer: MockWriter, formatter: MockFormatter) -> Self {
            Serializer { writer, formatter }
        }

        fn serialize_u128(self, value: u128) -> Result<(), std::io::Error> {
            self.formatter.begin_string(&mut self.writer)?;
            self.formatter.write_u128(&mut self.writer, value)?;
            self.formatter.end_string(&mut self.writer)
        }
    }

    let serializer = Serializer::new(MockWriter, MockFormatter::new(true));
    let result = serializer.serialize_u128(42);
    assert!(result.is_err());
}

