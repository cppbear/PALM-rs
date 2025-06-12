// Answer 0

#[test]
fn test_serialize_i16_success() {
    struct MockWriter;
    struct MockFormatter {
        begin_called: bool,
        write_called: bool,
    }

    impl MockFormatter {
        fn new() -> Self {
            Self {
                begin_called: false,
                write_called: false,
            }
        }

        fn begin_string(&mut self, _writer: &mut MockWriter) -> Result<(), std::io::Error> {
            self.begin_called = true;
            Ok(())
        }

        fn write_i16(&mut self, _writer: &mut MockWriter, _value: i16) -> Result<(), std::io::Error> {
            self.write_called = true;
            Ok(())
        }

        fn end_string(&mut self, _writer: &mut MockWriter) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct Serializer {
        writer: MockWriter,
        formatter: MockFormatter,
    }

    struct TestStruct {
        ser: Serializer,
    }

    impl TestStruct {
        fn serialize_i16(self, value: i16) -> Result<()> {
            tri!(self.ser.formatter.begin_string(&mut self.ser.writer).map_err(Error::io));
            tri!(self.ser.formatter.write_i16(&mut self.ser.writer, value).map_err(Error::io));
            self.ser.formatter.end_string(&mut self.ser.writer).map_err(Error::io)
        }
    }

    let formatter = MockFormatter::new();
    let writer = MockWriter;
    let serializer = Serializer { writer, formatter };
    let test_struct = TestStruct { ser: serializer };

    let result = test_struct.serialize_i16(16);
    assert!(result.is_ok());
}

#[test]
fn test_serialize_i16_write_error() {
    struct MockWriter;
    struct MockFormatter {
        begin_called: bool,
        write_error: bool,
    }

    impl MockFormatter {
        fn new() -> Self {
            Self {
                begin_called: false,
                write_error: false,
            }
        }

        fn begin_string(&mut self, _writer: &mut MockWriter) -> Result<(), std::io::Error> {
            self.begin_called = true;
            Ok(())
        }

        fn write_i16(&mut self, _writer: &mut MockWriter, _value: i16) -> Result<(), std::io::Error> {
            if self.write_error {
                Err(std::io::Error::new(std::io::ErrorKind::Other, "write error"))
            } else {
                Ok(())
            }
        }

        fn end_string(&mut self, _writer: &mut MockWriter) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct Serializer {
        writer: MockWriter,
        formatter: MockFormatter,
    }

    struct TestStruct {
        ser: Serializer,
    }

    impl TestStruct {
        fn serialize_i16(self, value: i16) -> Result<()> {
            tri!(self.ser.formatter.begin_string(&mut self.ser.writer).map_err(Error::io));
            tri!(self.ser.formatter.write_i16(&mut self.ser.writer, value).map_err(Error::io));
            self.ser.formatter.end_string(&mut self.ser.writer).map_err(Error::io)
        }
    }

    let mut formatter = MockFormatter::new();
    formatter.write_error = true;
    let writer = MockWriter;
    let serializer = Serializer { writer, formatter };
    let test_struct = TestStruct { ser: serializer };

    let result = test_struct.serialize_i16(16);
    assert!(result.is_err());
}

