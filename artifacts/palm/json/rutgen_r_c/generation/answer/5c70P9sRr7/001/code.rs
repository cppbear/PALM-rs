// Answer 0

#[test]
fn test_end_with_state_rest_should_return_err() {
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
            Err(Error::new(ErrorCode::Io, "mock error"))
        }

        fn end_object_value(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }

        fn end_object(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter;
    let mut formatter = MockFormatter;

    let ser = Serializer { writer, formatter };
    let state = State::Rest;

    let compound = Compound::Map { ser: &mut ser, state };

    let result = compound.end();
    assert!(result.is_err());
}

#[test]
fn test_end_with_state_rest_and_end_object_should_return_err() {
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
            Err(Error::new(ErrorCode::Io, "mock error"))
        }

        fn end_object(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter;
    let mut formatter = MockFormatter;

    let ser = Serializer { writer, formatter };
    let state = State::Rest;

    let compound = Compound::Map { ser: &mut ser, state };

    let result = compound.end();
    assert!(result.is_err());
}

