// Answer 0

#[test]
fn test_serialize_i32_begin_string_err() {
    struct MockFormatter {
        should_fail: bool,
    }

    impl MockFormatter {
        fn begin_string(&mut self, _: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
            if self.should_fail {
                Err(std::io::Error::new(std::io::ErrorKind::Other, "begin_string error"))
            } else {
                Ok(())
            }
        }

        fn write_i32(&mut self, _: &mut dyn std::io::Write, _: i32) -> Result<(), std::io::Error> {
            Ok(())
        }

        fn end_string(&mut self, _: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct MockWriter;

    impl std::io::Write for MockWriter {
        fn write(&mut self, _: &[u8]) -> Result<usize, std::io::Error> {
            Ok(0)
        }

        fn flush(&mut self) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct Serializer {
        writer: MockWriter,
        formatter: MockFormatter,
    }

    struct Ser {
        ser: Serializer,
    }

    impl Ser {
        fn serialize_i32(self, value: i32) -> Result<(), Error> {
            tri!(self.ser.formatter.begin_string(&mut self.ser.writer).map_err(Error::io));
            tri!(self.ser.formatter.write_i32(&mut self.ser.writer, value).map_err(Error::io));
            self.ser.formatter.end_string(&mut self.ser.writer).map_err(Error::io)
        }
    }

    let ser = Ser {
        ser: Serializer {
            writer: MockWriter,
            formatter: MockFormatter { should_fail: true },
        },
    };

    let result = ser.serialize_i32(42);
    assert!(result.is_err());
}

