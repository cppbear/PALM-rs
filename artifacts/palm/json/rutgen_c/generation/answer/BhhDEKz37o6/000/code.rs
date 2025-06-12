// Answer 0

#[test]
fn test_end_with_empty_state() {
    struct MockWriter;

    impl io::Write for MockWriter {
        fn write(&mut self, _: &[u8]) -> Result<usize> {
            Ok(0)
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn end_object(&mut self, _: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }

        fn end_object_value(&mut self, _: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let mut mock_writer = MockWriter;
    let mock_formatter = MockFormatter;

    let compound = Compound::Map {
        ser: &mut Serializer {
            writer: mock_writer,
            formatter: mock_formatter,
        },
        state: State::Empty,
    };

    let result = compound.end();
    assert!(result.is_ok());
}

#[test]
fn test_end_with_non_empty_state() {
    struct MockWriter;

    impl io::Write for MockWriter {
        fn write(&mut self, _: &[u8]) -> Result<usize> {
            Ok(0)
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn end_object(&mut self, _: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }

        fn end_object_value(&mut self, _: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let mut mock_writer = MockWriter;
    let mock_formatter = MockFormatter;

    let compound = Compound::Map {
        ser: &mut Serializer {
            writer: mock_writer,
            formatter: mock_formatter,
        },
        state: State::First,
    };

    let result = compound.end();
    assert!(result.is_ok());
}

