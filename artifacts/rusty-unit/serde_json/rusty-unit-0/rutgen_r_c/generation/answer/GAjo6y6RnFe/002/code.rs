// Answer 0

#[test]
fn test_serialize_i128_success() {
    struct MockFormatter;
    
    impl MockFormatter {
        fn begin_string(&mut self, _writer: &mut Vec<u8>) -> Result<()> {
            Ok(())
        }

        fn write_i128(&mut self, _writer: &mut Vec<u8>, _value: i128) -> Result<()> {
            Ok(())
        }

        fn end_string(&mut self, _writer: &mut Vec<u8>) -> Result<()> {
            Ok(())
        }
    }
    
    struct MockWriter(Vec<u8>);
    
    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.0.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    struct MockSerializer {
        writer: MockWriter,
        formatter: MockFormatter,
    }

    let mut writer = MockWriter(Vec::new());
    let formatter = MockFormatter;
    let serializer = MockSerializer { writer, formatter };
    let result = serializer.serialize_i128(1234567890123456789i128);

    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_serialize_i128_begin_string_fails() {
    struct MockFormatter;

    impl MockFormatter {
        fn begin_string(&mut self, _writer: &mut Vec<u8>) -> Result<()> {
            Err(Error)
        }

        fn write_i128(&mut self, _writer: &mut Vec<u8>, _value: i128) -> Result<()> {
            Ok(())
        }

        fn end_string(&mut self, _writer: &mut Vec<u8>) -> Result<()> {
            Ok(())
        }
    }

    struct MockWriter(Vec<u8>);

    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
            Ok(0)
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    struct MockSerializer {
        writer: MockWriter,
        formatter: MockFormatter,
    }

    let mut writer = MockWriter(Vec::new());
    let formatter = MockFormatter;
    let serializer = MockSerializer { writer, formatter };
    
    let _ = serializer.serialize_i128(123);
}

#[test]
#[should_panic]
fn test_serialize_i128_write_fails() {
    struct MockFormatter;

    impl MockFormatter {
        fn begin_string(&mut self, _writer: &mut Vec<u8>) -> Result<()> {
            Ok(())
        }

        fn write_i128(&mut self, _writer: &mut Vec<u8>, _value: i128) -> Result<()> {
            Err(Error)
        }

        fn end_string(&mut self, _writer: &mut Vec<u8>) -> Result<()> {
            Ok(())
        }
    }

    struct MockWriter(Vec<u8>);

    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
            Ok(0)
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    struct MockSerializer {
        writer: MockWriter,
        formatter: MockFormatter,
    }

    let mut writer = MockWriter(Vec::new());
    let formatter = MockFormatter;
    let serializer = MockSerializer { writer, formatter };
    
    let _ = serializer.serialize_i128(123);
}

