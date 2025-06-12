// Answer 0

#[test]
fn test_end_empty_map() {
    struct MockWriter;
    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn end_array(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
        fn end_object_value(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
        fn end_object(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let writer = MockWriter;
    let formatter = MockFormatter;
    let ser = Serializer { writer, formatter };
    let compound = Compound::Map { ser: &mut ser, state: State::Empty };

    assert!(compound.end().is_ok());
}

#[test]
fn test_end_non_empty_map() {
    struct MockWriter;
    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn end_array(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
        fn end_object_value(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
        fn end_object(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let writer = MockWriter;
    let formatter = MockFormatter;
    let mut ser = Serializer { writer, formatter };
    let compound = Compound::Map { ser: &mut ser, state: State::First };

    assert!(compound.end().is_ok());
}

