// Answer 0

#[test]
fn test_end_with_state_empty_and_err() {
    struct MockWriter;
    struct MockFormatter;

    impl MockFormatter {
        fn end_object_value(&self, _writer: &mut MockWriter) -> Result<()> {
            Err(Error::io(std::io::Error::new(std::io::ErrorKind::Other, "Mock error")))
        }
        
        fn end_object(&self, _writer: &mut MockWriter) -> Result<()> {
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

    struct Error {
        kind: std::io::Error,
    }

    impl Error {
        fn io(err: std::io::Error) -> Self {
            Error { kind: err }
        }
    }

    type Result<T> = std::result::Result<T, Error>;

    let ser = MockSerializer {
        formatter: MockFormatter,
        writer: MockWriter,
    };

    let compound = Compound::Map { ser, state: State::Empty };
    
    let result = match compound {
        Compound::Map { ser, state } => {
            match state {
                State::Empty => {}
                _ => unreachable!(),
            }
            match ser.formatter.end_object_value(&mut ser.writer).map_err(Error::io) {
                Ok(_) => unreachable!(),
                Err(err) => Err(err), 
            }
        }
    };

    if let Err(err) = result {
        assert_eq!(err.kind.to_string(), "Mock error");
    } else {
        panic!("Expected an error, but got Ok");
    }
}

