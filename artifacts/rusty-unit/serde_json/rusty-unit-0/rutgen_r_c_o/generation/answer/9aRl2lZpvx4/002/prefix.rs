// Answer 0

#[test]
fn test_end_with_empty_state() {
    struct DummyWriter;

    impl io::Write for DummyWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let writer = DummyWriter;
    let serializer = Serializer { writer };
    let compound = Compound::Map {
        ser: &mut serializer,
        state: State::Empty,
    };

    let _ = compound.end();
}

#[test]
fn test_end_with_first_state() {
    struct DummyWriter;

    impl io::Write for DummyWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let writer = DummyWriter;
    let serializer = Serializer { writer };
    let compound = Compound::Map {
        ser: &mut serializer,
        state: State::First,
    };

    let _ = compound.end();
}

#[test]
fn test_end_with_rest_state() {
    struct DummyWriter;

    impl io::Write for DummyWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let writer = DummyWriter;
    let serializer = Serializer { writer };
    let compound = Compound::Map {
        ser: &mut serializer,
        state: State::Rest,
    };

    let _ = compound.end();
}

