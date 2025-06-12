// Answer 0

#[test]
fn test_serialize_i128_begin_string_err() {
    struct MockWriter {
        should_error: bool,
    }

    struct MockFormatter {
        writer: MockWriter,
    }

    struct MockSer {
        formatter: MockFormatter,
    }

    struct Serializer {
        ser: MockSer,
    }

    impl MockWriter {
        fn begin_string(&mut self) -> Result<(), std::io::Error> {
            if self.should_error {
                Err(std::io::Error::new(std::io::ErrorKind::Other, "mock error"))
            } else {
                Ok(())
            }
        }

        fn write_i128(&mut self, _value: i128) -> Result<(), std::io::Error> {
            Ok(())
        }

        fn end_string(&mut self) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    impl MockFormatter {
        fn begin_string(&mut self, _: &mut MockWriter) -> Result<(), std::io::Error> {
            self.writer.begin_string()
        }

        fn write_i128(&mut self, writer: &mut MockWriter, value: i128) -> Result<(), std::io::Error> {
            writer.write_i128(value)
        }

        fn end_string(&mut self, _: &mut MockWriter) -> Result<(), std::io::Error> {
            self.writer.end_string()
        }
    }

    impl Serializer {
        fn serialize_i128(self, value: i128) -> Result<()> {
            tri!(self.ser.formatter.begin_string(&mut self.ser.writer).map_err(Error::io));
            tri!(self.ser.formatter.write_i128(&mut self.ser.writer, value).map_err(Error::io));
            self.ser.formatter.end_string(&mut self.ser.writer).map_err(Error::io)
        }
    }

    let writer = MockWriter { should_error: true };
    let formatter = MockFormatter { writer };
    let ser = MockSer { formatter };
    let serializer = Serializer { ser };

    let result = serializer.serialize_i128(123456789012345678901234567890i128);
    assert!(result.is_err());
}

