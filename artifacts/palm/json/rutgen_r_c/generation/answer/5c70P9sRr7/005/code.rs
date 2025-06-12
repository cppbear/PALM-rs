// Answer 0

#[test]
fn test_end_with_empty_state() {
    struct MockWriter {
        buffer: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> core::result::Result<usize, std::io::Error> {
            self.buffer.extend_from_slice(buf);
            Ok(buf.len())
        }
        
        fn flush(&mut self) -> core::result::Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn end_array(&mut self, writer: &mut impl io::Write) -> Result<()> {
            writer.write(&[b']'])?;
            Ok(())
        }
        
        fn end_object_value(&mut self, writer: &mut impl io::Write) -> Result<()> {
            writer.write(&[b'}'])?;
            Ok(())
        }
        
        fn end_object(&mut self, writer: &mut impl io::Write) -> Result<()> {
            writer.write(&[b'}'])?;
            Ok(())
        }
    }

    // Given
    let writer = MockWriter { buffer: Vec::new() };
    let formatter = MockFormatter;
    let state = State::Empty;

    let mut compound = Compound::Map {
        ser: &mut Serializer { writer, formatter },
        state,
    };

    // When
    let result = compound.end();

    // Then
    assert!(result.is_ok());
    assert!(matches!(result, Ok(())));
}

