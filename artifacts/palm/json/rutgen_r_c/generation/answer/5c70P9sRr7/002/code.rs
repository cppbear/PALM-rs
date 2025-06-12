// Answer 0

#[test]
fn test_end_with_state_rest_success() {
    struct MockWriter;
    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> core::result::Result<usize, std::io::Error> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> core::result::Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct MockFormatter;
    impl Formatter for MockFormatter {
        fn end_array(&mut self, _writer: &mut impl io::Write) -> Result<()> {
            Ok(())
        }
        fn end_object_value(&mut self, _writer: &mut impl io::Write) -> Result<()> {
            Ok(())
        }
        fn end_object(&mut self, _writer: &mut impl io::Write) -> Result<()> {
            Ok(())
        }
    }

    let writer = MockWriter;
    let formatter = MockFormatter;
    let mut serializer = Serializer { writer, formatter };

    let compound = Compound::Map { ser: &mut serializer, state: State::Rest };
    let result = compound.end();

    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_end_with_state_rest_end_object_value_err() {
    struct MockWriter;
    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> core::result::Result<usize, std::io::Error> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> core::result::Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct MockFormatter;
    impl Formatter for MockFormatter {
        fn end_array(&mut self, _writer: &mut impl io::Write) -> Result<()> {
            Ok(())
        }
        fn end_object_value(&mut self, _writer: &mut impl io::Write) -> Result<()> {
            Err(Error::new(ErrorCode::SomeError)) // Trigger error case
        }
        fn end_object(&mut self, _writer: &mut impl io::Write) -> Result<()> {
            Ok(())
        }
    }

    let writer = MockWriter;
    let formatter = MockFormatter;
    let mut serializer = Serializer { writer, formatter };

    let compound = Compound::Map { ser: &mut serializer, state: State::Rest };
    let _ = compound.end(); // This should panic
}

#[test]
fn test_end_with_state_empty() {
    struct MockWriter;
    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> core::result::Result<usize, std::io::Error> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> core::result::Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct MockFormatter;
    impl Formatter for MockFormatter {
        fn end_array(&mut self, _writer: &mut impl io::Write) -> Result<()> {
            Ok(())
        }
        fn end_object_value(&mut self, _writer: &mut impl io::Write) -> Result<()> {
            Ok(())
        }
        fn end_object(&mut self, _writer: &mut impl io::Write) -> Result<()> {
            Ok(())
        }
    }

    let writer = MockWriter;
    let formatter = MockFormatter;
    let mut serializer = Serializer { writer, formatter };

    let compound = Compound::Map { ser: &mut serializer, state: State::Empty };
    let result = compound.end();

    assert!(result.is_ok());
}

