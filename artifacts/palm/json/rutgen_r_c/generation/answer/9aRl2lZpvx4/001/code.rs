// Answer 0

#[test]
fn test_end_with_non_empty_state() {
    struct MockWriter;
    struct MockFormatter;

    impl MockFormatter {
        fn end_object(&self, _writer: &mut MockWriter) -> Result<()> {
            // Simulate successful end_object call
            Ok(())
        }
    }

    let writer = MockWriter;
    let formatter = MockFormatter;
    
    let mut serializer = Serializer {
        writer,
        formatter,
    };

    let state = State::First; // Choose a non-empty state
    let compound = Compound::Map {
        ser: &mut serializer,
        state,
    };

    let result = compound.end();
    assert!(result.is_ok());
}

#[test]
fn test_end_with_empty_state() {
    struct MockWriter;
    struct MockFormatter;

    impl MockFormatter {
        fn end_object(&self, _writer: &mut MockWriter) -> Result<()> {
            // Simulate successful end_object call
            Ok(())
        }
    }

    let writer = MockWriter;
    let formatter = MockFormatter;
    
    let mut serializer = Serializer {
        writer,
        formatter,
    };

    let state = State::Empty; // Choosing the empty state
    let compound = Compound::Map {
        ser: &mut serializer,
        state,
    };

    let result = compound.end();
    assert!(result.is_ok());
}

