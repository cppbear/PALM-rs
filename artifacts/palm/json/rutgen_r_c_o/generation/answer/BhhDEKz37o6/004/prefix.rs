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
        fn end_object(&mut self, _: &mut MockWriter) -> Result<()> {
            Ok(())
        }
        fn end_object_value(&mut self, _: &mut MockWriter) -> Result<()> {
            Err(Error::from(ErrorCode::SomeError))
        }
    }

    let writer = MockWriter {};
    let formatter = MockFormatter {};
    let serializer = Serializer {
        writer,
        formatter,
    };
    let compound = Compound::Map {
        ser: &mut serializer,
        state: State::Empty,
    };

    let result = compound.end();
}

#[test]
fn test_end_with_empty_state_and_error() {
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
        fn end_object(&mut self, _: &mut MockWriter) -> Result<()> {
            Ok(())
        }
        fn end_object_value(&mut self, _: &mut MockWriter) -> Result<()> {
            Err(Error::from(ErrorCode::SomeError))
        }
    }

    let writer = MockWriter {};
    let formatter = MockFormatter {};
    let serializer = Serializer {
        writer,
        formatter,
    };
    let compound = Compound::Map {
        ser: &mut serializer,
        state: State::Empty,
    };

    let result = compound.end();
}

