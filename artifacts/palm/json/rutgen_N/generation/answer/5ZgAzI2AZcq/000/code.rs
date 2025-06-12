// Answer 0

#[test]
fn test_serialize_i16_success() {
    struct MockWriter {
        data: Vec<u8>,
    }

    impl MockWriter {
        fn new() -> Self {
            MockWriter { data: Vec::new() }
        }

        fn write(&mut self, buf: &[u8]) -> Result<usize, std::io::Error> {
            self.data.extend_from_slice(buf);
            Ok(buf.len())
        }
    }

    struct MockFormatter<'a> {
        writer: &'a mut MockWriter,
    }

    impl<'a> MockFormatter<'a> {
        fn begin_string(&mut self) -> Result<(), std::io::Error> {
            self.writer.write(b"\"")?;
            Ok(())
        }

        fn write_i16(&mut self, value: i16) -> Result<(), std::io::Error> {
            let bytes = value.to_string().as_bytes();
            self.writer.write(bytes)?;
            Ok(())
        }

        fn end_string(&mut self) -> Result<(), std::io::Error> {
            self.writer.write(b"\"")?;
            Ok(())
        }
    }

    struct Serializer {
        writer: MockWriter,
        formatter: MockFormatter<'static>,
    }

    impl Serializer {
        fn new() -> Self {
            let writer = MockWriter::new();
            let formatter = MockFormatter { writer: &mut writer };
            Serializer { writer, formatter }
        }

        fn serialize_i16(&mut self, value: i16) -> Result<(), std::io::Error> {
            self.formatter.begin_string()?;
            self.formatter.write_i16(value)?;
            self.formatter.end_string()?;
            Ok(())
        }
    }

    let mut serializer = Serializer::new();
    let result = serializer.serialize_i16(42);
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_serialize_i16_panics_on_io_error() {
    struct FailingWriter;

    impl FailingWriter {
        fn write(&self, _buf: &[u8]) -> Result<usize, std::io::Error> {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "IO error"))
        }
    }

    struct FailingFormatter<'a> {
        writer: &'a FailingWriter,
    }

    impl<'a> FailingFormatter<'a> {
        fn begin_string(&mut self) -> Result<(), std::io::Error> {
            self.writer.write(b"\"")?;
            Ok(())
        }

        fn write_i16(&mut self, _value: i16) -> Result<(), std::io::Error> {
            self.writer.write(b"");
        }

        fn end_string(&mut self) -> Result<(), std::io::Error> {
            self.writer.write(b"\"")?;
            Ok(())
        }
    }

    struct FailingSerializer {
        writer: FailingWriter,
        formatter: FailingFormatter<'static>,
    }

    impl FailingSerializer {
        fn new() -> Self {
            let writer = FailingWriter;
            let formatter = FailingFormatter { writer: &writer };
            FailingSerializer { writer, formatter }
        }

        fn serialize_i16(&mut self, value: i16) -> Result<(), std::io::Error> {
            self.formatter.begin_string()?;
            self.formatter.write_i16(value)?;
            self.formatter.end_string()?;
            Ok(())
        }
    }

    let mut serializer = FailingSerializer::new();
    let _ = serializer.serialize_i16(42).expect("Should panic on IO error");
}

