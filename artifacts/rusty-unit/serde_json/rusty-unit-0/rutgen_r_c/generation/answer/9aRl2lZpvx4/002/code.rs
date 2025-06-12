// Answer 0

#[test]
fn test_end_with_state_empty() {
    struct DummyWriter;

    impl io::Write for DummyWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct DummyFormatter;

    impl Formatter for DummyFormatter {
        fn end_object(self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let writer = DummyWriter;
    let formatter = DummyFormatter;
    
    let mut serializer = Serializer {
        writer,
        formatter,
    };
    
    let compound = Compound::Map {
        ser: &mut serializer,
        state: State::Empty,
    };
    
    let result = compound.end();
    assert!(result.is_ok());
}

