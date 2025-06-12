// Answer 0

#[test]
fn test_serialize_u16_begin_string_err() {
    struct MockWriter {
        error_on_begin: bool,
    }

    impl MockWriter {
        fn new(error_on_begin: bool) -> Self {
            Self { error_on_begin }
        }
    }

    struct MockFormatter<'a> {
        writer: &'a mut MockWriter,
    }

    impl<'a> MockFormatter<'a> {
        fn begin_string(&mut self, _: &mut MockWriter) -> Result<(), std::io::Error> {
            if self.writer.error_on_begin {
                Err(std::io::Error::new(std::io::ErrorKind::Other, "begin_string error"))
            } else {
                Ok(())
            }
        }

        fn write_u16(&mut self, _: &mut MockWriter, _: u16) -> Result<(), std::io::Error> {
            Ok(())
        }

        fn end_string(&mut self, _: &mut MockWriter) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct MockSer {
        formatter: MockFormatter<'static>,
    }

    impl MockSer {
        fn new(writer: &'static mut MockWriter) -> Self {
            Self {
                formatter: MockFormatter { writer },
            }
        }
    }

    struct Serializer {
        ser: MockSer,
    }

    impl Serializer {
        fn new(writer: &'static mut MockWriter) -> Self {
            Self {
                ser: MockSer::new(writer),
            }
        }

        fn serialize_u16(self, value: u16) -> Result<(), std::io::Error> {
            tri!(self.ser.formatter.begin_string(&mut self.ser.writer).map_err(Error::io));
            tri!(self.ser.formatter.write_u16(&mut self.ser.writer, value).map_err(Error::io));
            self.ser.formatter.end_string(&mut self.ser.writer).map_err(Error::io)
        }
    }

    let mut mock_writer = MockWriter::new(true);
    let serializer = Serializer::new(&mut mock_writer);
    let result = serializer.serialize_u16(100u16);
    
    assert!(result.is_err());
}

