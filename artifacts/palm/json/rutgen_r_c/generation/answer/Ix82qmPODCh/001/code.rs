// Answer 0

#[test]
fn test_end_with_empty_state() {
    struct TestWriter;
    impl io::Write for TestWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct TestFormatter;

    let mut serializer = Serializer {
        writer: TestWriter,
        formatter: TestFormatter,
    };

    let compound = Compound::Map {
        ser: &mut serializer,
        state: State::Empty,
    };

    assert!(matches!(compound.end(), Ok(())));
}

#[test]
fn test_end_with_first_state() {
    struct TestWriter;
    impl io::Write for TestWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct TestFormatter;

    let mut serializer = Serializer {
        writer: TestWriter,
        formatter: TestFormatter,
    };

    let compound = Compound::Map {
        ser: &mut serializer,
        state: State::First,
    };

    assert!(matches!(compound.end(), Ok(())));
}

#[test]
fn test_end_with_rest_state() {
    struct TestWriter;
    impl io::Write for TestWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct TestFormatter;

    let mut serializer = Serializer {
        writer: TestWriter,
        formatter: TestFormatter,
    };

    let compound = Compound::Map {
        ser: &mut serializer,
        state: State::Rest,
    };

    assert!(matches!(compound.end(), Ok(())));
}

#[should_panic]
#[test]
fn test_end_should_panic_with_invalid_state() {
    struct TestWriter;
    impl io::Write for TestWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct TestFormatter;

    let mut serializer = Serializer {
        writer: TestWriter,
        formatter: TestFormatter,
    };

    let compound = Compound::Map {
        ser: &mut serializer,
        state: State::Empty,
    };

    // Trigger panic with an invalid call.
    let _ = compound.end();
}

