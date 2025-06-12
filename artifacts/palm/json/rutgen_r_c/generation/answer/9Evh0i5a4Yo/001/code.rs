// Answer 0

#[test]
fn test_serialize_bool_true() {
    struct MockWriter {
        written: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.written.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn write_all(&mut self, buf: &[u8]) -> Result<()> {
            self.write(buf)?;
            Ok(())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl MockFormatter {
        fn write_bool(&mut self, writer: &mut dyn io::Write, value: bool) -> Result<()> {
            let byte = if value { b't' } else { b'f' };
            writer.write_all(&[byte])?;
            Ok(())
        }
    }

    let mut writer = MockWriter { written: Vec::new() };
    let formatter = MockFormatter;
    let serializer = Serializer { writer, formatter };
    
    serializer.serialize_bool(true).unwrap();
    assert_eq!(serializer.writer.written, &[b't']);
}

#[test]
fn test_serialize_bool_false() {
    struct MockWriter {
        written: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.written.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn write_all(&mut self, buf: &[u8]) -> Result<()> {
            self.write(buf)?;
            Ok(())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl MockFormatter {
        fn write_bool(&mut self, writer: &mut dyn io::Write, value: bool) -> Result<()> {
            let byte = if value { b't' } else { b'f' };
            writer.write_all(&[byte])?;
            Ok(())
        }
    }

    let mut writer = MockWriter { written: Vec::new() };
    let formatter = MockFormatter;
    let serializer = Serializer { writer, formatter };

    serializer.serialize_bool(false).unwrap();
    assert_eq!(serializer.writer.written, &[b'f']);
}

#[should_panic]
#[test]
fn test_serialize_bool_io_error() {
    struct MockWriter;

    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Err(Error::from(ErrorCode::IoError))
        }

        fn write_all(&mut self, buf: &[u8]) -> Result<()> {
            self.write(buf)?;
            Ok(())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl MockFormatter {
        fn write_bool(&mut self, _writer: &mut dyn io::Write, _value: bool) -> Result<()> {
            // No implementation needed for this test
            Ok(())
        }
    }

    let writer = MockWriter;
    let formatter = MockFormatter;
    let serializer = Serializer { writer, formatter };

    serializer.serialize_bool(true).unwrap();
}

