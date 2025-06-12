// Answer 0

#[test]
fn test_end_with_non_empty_state() {
    struct MockWriter;
    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;
    impl Formatter for MockFormatter {
        fn end_object(&mut self, writer: &mut dyn io::Write) -> Result<()> {
            writer.write(&[b'}'])?;
            Ok(())
        }

        fn end_object_value(&mut self, writer: &mut dyn io::Write) -> Result<()> {
            writer.write(&[b','])?;
            Ok(())
        }
    }

    let mut writer = MockWriter;
    let formatter = MockFormatter;

    let mut compound = Compound::Map {
        ser: &mut Serializer {
            writer,
            formatter,
        },
        state: State::Rest,
    };

    let result = compound.end();
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_end_with_empty_state() {
    struct MockWriter;
    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;
    impl Formatter for MockFormatter {
        fn end_object(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            unreachable!()
        }

        fn end_object_value(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            unreachable!()
        }
    }

    let writer = MockWriter;
    let formatter = MockFormatter;

    let compound = Compound::Map {
        ser: &mut Serializer {
            writer,
            formatter,
        },
        state: State::Empty,
    };

    // This should panic due to empty state
    compound.end();
}

#[test]
fn test_end_with_successive_calls() {
    struct MockWriter;
    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;
    impl Formatter for MockFormatter {
        fn end_object(&mut self, writer: &mut dyn io::Write) -> Result<()> {
            writer.write(&[b'}'])?;
            Ok(())
        }

        fn end_object_value(&mut self, writer: &mut dyn io::Write) -> Result<()> {
            writer.write(&[b','])?;
            Ok(())
        }
    }

    let mut writer = MockWriter;
    let formatter = MockFormatter;

    let mut compound = Compound::Map {
        ser: &mut Serializer {
            writer,
            formatter,
        },
        state: State::Rest,
    };

    let result1 = compound.end();
    assert!(result1.is_ok());

    let result2 = compound.end();
    assert!(result2.is_ok());
}

