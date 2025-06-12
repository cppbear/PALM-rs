// Answer 0

#[test]
fn test_end_map_empty_state() {
    struct MockFormatter;
    struct MockWriter;

    impl MockFormatter {
        fn end_object(&self, _writer: &mut MockWriter) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct MockSerializer {
        formatter: MockFormatter,
        writer: MockWriter,
    }

    enum State {
        Empty,
        NonEmpty,
    }

    enum Compound {
        Map { ser: MockSerializer, state: State },
    }

    let serializer = MockSerializer {
        formatter: MockFormatter,
        writer: MockWriter,
    };

    let compound = Compound::Map {
        ser: serializer,
        state: State::Empty,
    };

    let result = match compound {
        Compound::Map { ser, state } => match state {
            State::Empty => Ok(()),
            _ => ser.formatter.end_object(&mut ser.writer).map_err(Error::io),
        },
    };
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_end_map_non_empty_state() {
    struct MockFormatter;
    struct MockWriter;

    impl MockFormatter {
        fn end_object(&self, _writer: &mut MockWriter) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct MockSerializer {
        formatter: MockFormatter,
        writer: MockWriter,
    }

    enum State {
        Empty,
        NonEmpty,
    }

    enum Compound {
        Map { ser: MockSerializer, state: State },
    }

    let serializer = MockSerializer {
        formatter: MockFormatter,
        writer: MockWriter,
    };

    let compound = Compound::Map {
        ser: serializer,
        state: State::NonEmpty,
    };

    let _result = match compound {
        Compound::Map { ser, state } => match state {
            State::Empty => Ok(()),
            _ => ser.formatter.end_object(&mut ser.writer).map_err(Error::io),
        },
    };
}

