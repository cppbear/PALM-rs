// Answer 0

#[test]
#[should_panic]
fn test_serialize_bool_empty_writer() {
    struct MockWriter {
        capacity: usize,
        data: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            if self.data.len() + buf.len() > self.capacity {
                return Err(Error::io(io::Error::new(io::ErrorKind::Other, "full")));
            }
            self.data.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn begin_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Err(Error::io(io::Error::new(io::ErrorKind::Other, "error during begin_string")))
        }

        fn write_bool(&mut self, _writer: &mut dyn io::Write, _value: bool) -> Result<()> {
            Ok(())
        }

        fn end_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let writer = MockWriter { capacity: 5, data: Vec::new() };
    let formatter = MockFormatter {};
    let serializer = Serializer { writer, formatter };

    let _ = serializer.serialize_bool(true);
}

#[test]
#[should_panic]
fn test_serialize_bool_full_writer() {
    struct MockWriter {
        data: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Err(Error::io(io::Error::new(io::ErrorKind::Other, "full")))
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn begin_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }

        fn write_bool(&mut self, _writer: &mut dyn io::Write, _value: bool) -> Result<()> {
            Err(Error::io(io::Error::new(io::ErrorKind::Other, "full")))
        }

        fn end_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let writer = MockWriter { data: Vec::new() };
    let formatter = MockFormatter {};
    let serializer = Serializer { writer, formatter };

    let _ = serializer.serialize_bool(true);
}

#[test]
fn test_serialize_bool_writer_error() {
    struct MockErrorWriter;

    impl io::Write for MockErrorWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Err(Error::io(io::Error::new(io::ErrorKind::Other, "write error")))
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn begin_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }

        fn write_bool(&mut self, _writer: &mut dyn io::Write, _value: bool) -> Result<()> {
            Ok(())
        }

        fn end_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let writer = MockErrorWriter;
    let formatter = MockFormatter {};
    let serializer = Serializer { writer, formatter };

    let _ = serializer.serialize_bool(false);
}

