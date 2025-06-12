// Answer 0

fn test_end_with_non_empty_state() {
    struct MockWriter;
    struct MockFormatter;

    impl MockFormatter {
        fn end_array(&self, _writer: &mut MockWriter) -> Result<()> {
            Ok(())
        }
    }

    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let writer = MockWriter;
    let formatter = MockFormatter;
    let mut compound = Compound::Map {
        ser: &mut Serializer { writer, formatter },
        state: State::First,
    };

    let result = compound.end();
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_end_with_empty_state() {
    let writer = MockWriter;
    let formatter = MockFormatter;
    let mut compound = Compound::Map {
        ser: &mut Serializer { writer, formatter },
        state: State::Empty,
    };

    // This should panic on the assertion that we shouldn't be in an empty state
    let _ = compound.end();
}

