// Answer 0

#[test]
fn test_end_empty_state() {
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
    }

    let writer = MockWriter;
    let formatter = MockFormatter;
    let serializer = Serializer { writer, formatter };
    
    let empty_state = State::Empty;
    let compound = Compound::Map { ser: &serializer, state: empty_state };

    let result = compound.end();
    assert!(result.is_ok());
    assert_eq!(result, Ok(()));
}

