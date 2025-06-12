// Answer 0

#[test]
fn test_end_with_state_first() {
    struct MockWriter;

    impl io::Write for MockWriter {
        fn write(&mut self, _: &[u8]) -> Result<usize> {
            Err(Error::new(ErrorCode::Custom))
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn end_array(&mut self, _: &mut dyn io::Write) -> Result<()> {
            Err(Error::new(ErrorCode::Custom))
        }

        fn end_object_value(&mut self, _: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }

        fn end_object(&mut self, _: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let writer = MockWriter;
    let formatter = MockFormatter;
    let serializer = Serializer { writer, formatter };
    let state = State::First;

    let compound = Compound::Map { ser: &mut serializer, state };

    let _ = compound.end();
}

#[test]
fn test_end_with_state_rest() {
    struct MockWriter;

    impl io::Write for MockWriter {
        fn write(&mut self, _: &[u8]) -> Result<usize> {
            Err(Error::new(ErrorCode::Custom))
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn end_array(&mut self, _: &mut dyn io::Write) -> Result<()> {
            Err(Error::new(ErrorCode::Custom))
        }

        fn end_object_value(&mut self, _: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }

        fn end_object(&mut self, _: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let writer = MockWriter;
    let formatter = MockFormatter;
    let serializer = Serializer { writer, formatter };
    let state = State::Rest;

    let compound = Compound::Map { ser: &mut serializer, state };

    let _ = compound.end();
}

