// Answer 0

#[test]
fn test_end_state_first() {
    struct TestWriter;
    impl io::Write for TestWriter {
        fn write(&mut self, _: &[u8]) -> Result<usize> {
            Ok(0)
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let writer = TestWriter;
    let formatter = CompactFormatter;
    let mut serializer = Serializer {
        writer,
        formatter,
    };
    let state = State::First;

    let compound = Compound::Map { ser: &mut serializer, state };
    compound.end();
}

#[test]
fn test_end_state_rest() {
    struct TestWriter;
    impl io::Write for TestWriter {
        fn write(&mut self, _: &[u8]) -> Result<usize> {
            Ok(0)
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let writer = TestWriter;
    let formatter = CompactFormatter;
    let mut serializer = Serializer {
        writer,
        formatter,
    };
    let state = State::Rest;

    let compound = Compound::Map { ser: &mut serializer, state };
    compound.end();
}

