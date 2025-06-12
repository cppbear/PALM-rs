// Answer 0

#[test]
fn test_end_with_state_first() {
    struct DummyWriter;
    
    impl io::Write for DummyWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let writer = DummyWriter;
    let formatter = CompactFormatter; // Assuming CompactFormatter is a default implementation
    let mut ser = Serializer { writer, formatter };
    let state = State::First;
    
    let compound = Compound::Map { ser: &mut ser, state };
    let result = compound.end();
}

#[test]
fn test_end_with_state_rest() {
    struct DummyWriter;
    
    impl io::Write for DummyWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let writer = DummyWriter;
    let formatter = CompactFormatter; // Assuming CompactFormatter is a default implementation
    let mut ser = Serializer { writer, formatter };
    let state = State::Rest;
    
    let compound = Compound::Map { ser: &mut ser, state };
    let result = compound.end();
}

