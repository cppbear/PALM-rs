// Answer 0

#[test]
fn test_end_empty_map() {
    struct TestWriter;
    impl io::Write for TestWriter {
        fn write(&mut self, _: &[u8]) -> Result<usize> {
            Ok(0)
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct TestFormatter;
    impl Formatter for TestFormatter {
        fn end_object(&self, _: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let writer = TestWriter;
    let formatter = TestFormatter;
    let serializer = Serializer { writer, formatter };
    let compound = Compound::Map { ser: &serializer, state: State::Empty };

    let result = compound.end();
    assert!(result.is_ok());
}

#[test]
fn test_end_non_empty_map() {
    struct TestWriter;
    impl io::Write for TestWriter {
        fn write(&mut self, _: &[u8]) -> Result<usize> {
            Ok(0)
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct TestFormatter;
    impl Formatter for TestFormatter {
        fn end_object(&self, _: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let writer = TestWriter;
    let formatter = TestFormatter;
    let serializer = Serializer { writer, formatter };
    let compound = Compound::Map { ser: &serializer, state: State::Rest };

    let result = compound.end();
    assert!(result.is_ok());
}

