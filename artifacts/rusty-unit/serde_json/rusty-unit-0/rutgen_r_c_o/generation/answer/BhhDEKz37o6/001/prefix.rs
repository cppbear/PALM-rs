// Answer 0

#[test]
#[should_panic]
fn test_end_with_state_first_and_invalid_writer() {
    struct InvalidWriter;

    impl io::Write for InvalidWriter {
        fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
            Err(io::Error::new(io::ErrorKind::Other, "invalid write"))
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let invalid_writer = InvalidWriter;
    let formatter = CompactFormatter {};
    
    let mut serializer = Serializer {
        writer: invalid_writer,
        formatter,
    };

    let compound = Compound::Map {
        ser: &mut serializer,
        state: State::First,
    };

    compound.end();  // This should panic due to the invalid writer.
}

#[test]
#[should_panic]
fn test_end_with_state_rest_and_invalid_writer() {
    struct InvalidWriter;

    impl io::Write for InvalidWriter {
        fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
            Err(io::Error::new(io::ErrorKind::Other, "invalid write"))
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let invalid_writer = InvalidWriter;
    let formatter = CompactFormatter {};

    let mut serializer = Serializer {
        writer: invalid_writer,
        formatter,
    };

    let compound = Compound::Map {
        ser: &mut serializer,
        state: State::Rest,
    };

    compound.end();  // This should panic due to the invalid writer.
}

