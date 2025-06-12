// Answer 0

#[test]
fn test_serialize_bool_begin_string_err() {
    struct MockFormatter {
        should_fail_begin: bool,
    }

    impl MockFormatter {
        fn begin_string(&self, _writer: &mut dyn std::io::Write) -> Result<()> {
            if self.should_fail_begin {
                Err(std::io::Error::new(std::io::ErrorKind::Other, "begin_string failed").into())
            } else {
                Ok(())
            }
        }

        fn write_bool(&self, _writer: &mut dyn std::io::Write, _value: bool) -> Result<()> {
            Ok(())
        }

        fn end_string(&self, _writer: &mut dyn std::io::Write) -> Result<()> {
            Ok(())
        }
    }

    struct MockSer {
        writer: Vec<u8>,
        formatter: MockFormatter,
    }

    struct MockSerializer {
        ser: MockSer,
    }

    impl MockSerializer {
        fn serialize_bool(self, value: bool) -> Result<()> {
            tri!(self
                .ser
                .formatter
                .begin_string(&mut self.ser.writer)
                .map_err(Error::io));
            tri!(self
                .ser
                .formatter
                .write_bool(&mut self.ser.writer, value)
                .map_err(Error::io));
            self.ser
                .formatter
                .end_string(&mut self.ser.writer)
                .map_err(Error::io)
        }
    }

    let serializer = MockSerializer {
        ser: MockSer {
            writer: Vec::new(),
            formatter: MockFormatter { should_fail_begin: true },
        },
    };

    let result = serializer.serialize_bool(true);
    assert!(result.is_err());
}


