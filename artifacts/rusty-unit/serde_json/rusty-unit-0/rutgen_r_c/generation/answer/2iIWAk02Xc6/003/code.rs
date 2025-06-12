// Answer 0

#[test]
fn test_serialize_seq_empty_array() {
    struct MockWriter {
        data: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.data.extend_from_slice(buf);
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

    impl Formatter for MockFormatter {
        fn begin_array(&mut self, writer: &mut dyn io::Write) -> Result<()> {
            writer.write_all(b"[")?;
            Ok(())
        }

        fn end_array(&mut self, writer: &mut dyn io::Write) -> Result<()> {
            writer.write_all(b"]")?;
            Ok(())
        }
    }

    let mut writer = MockWriter { data: Vec::new() };
    let formatter = MockFormatter;
    let mut serializer = Serializer { writer, formatter };

    let result = serializer.serialize_seq(Some(0)).unwrap();
    assert!(matches!(result, Compound::Map { state: State::Empty, .. }));
    assert_eq!(serializer.writer.data, b"[]");
}

#[test]
fn test_serialize_seq_non_empty_array() {
    struct MockWriter {
        data: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.data.extend_from_slice(buf);
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

    impl Formatter for MockFormatter {
        fn begin_array(&mut self, writer: &mut dyn io::Write) -> Result<()> {
            writer.write_all(b"[")?;
            Ok(())
        }

        fn end_array(&mut self, writer: &mut dyn io::Write) -> Result<()> {
            writer.write_all(b"]")?;
            Ok(())
        }
    }

    let mut writer = MockWriter { data: Vec::new() };
    let formatter = MockFormatter;
    let mut serializer = Serializer { writer, formatter };

    let result = serializer.serialize_seq(Some(5)).unwrap();
    assert!(matches!(result, Compound::Map { state: State::First, .. }));
    assert_eq!(serializer.writer.data, b"[");
}

