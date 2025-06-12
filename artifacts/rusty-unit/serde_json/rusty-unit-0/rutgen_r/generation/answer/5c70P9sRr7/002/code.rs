// Answer 0

#[test]
fn test_end_with_state_not_empty_and_ok() {
    struct MockFormatter;
    struct MockWriter;

    impl MockFormatter {
        fn end_array(&self, _: &mut MockWriter) -> Result<(), std::io::Error> {
            Ok(())
        }

        fn end_object_value(&self, _: &mut MockWriter) -> Result<(), std::io::Error> {
            Ok(())
        }

        fn end_object(&self, _: &mut MockWriter) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct MockSer {
        formatter: MockFormatter,
        writer: MockWriter,
    }

    enum State {
        NonEmpty,
        Empty,
    }

    enum Compound {
        Map { ser: MockSer, state: State },
    }

    fn end(self: Compound) -> Result<(), std::io::Error> {
        match self {
            Compound::Map { ser, state } => {
                match state {
                    State::Empty => {}
                    _ => ser.formatter.end_array(&mut ser.writer)?,
                }
                ser.formatter.end_object_value(&mut ser.writer)?;
                ser.formatter.end_object(&mut ser.writer)
            }
        }
    }

    let ser = MockSer {
        formatter: MockFormatter,
        writer: MockWriter,
    };
    let compound = Compound::Map {
        ser,
        state: State::NonEmpty,
    };

    let result = end(compound);
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_end_with_state_not_empty_and_err() {
    struct MockFormatter;
    struct MockWriter;

    impl MockFormatter {
        fn end_array(&self, _: &mut MockWriter) -> Result<(), std::io::Error> {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "error"))
        }

        fn end_object_value(&self, _: &mut MockWriter) -> Result<(), std::io::Error> {
            Ok(())
        }

        fn end_object(&self, _: &mut MockWriter) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct MockSer {
        formatter: MockFormatter,
        writer: MockWriter,
    }

    enum State {
        NonEmpty,
        Empty,
    }

    enum Compound {
        Map { ser: MockSer, state: State },
    }

    fn end(self: Compound) -> Result<(), std::io::Error> {
        match self {
            Compound::Map { ser, state } => {
                match state {
                    State::Empty => {}
                    _ => ser.formatter.end_array(&mut ser.writer)?,
                }
                ser.formatter.end_object_value(&mut ser.writer)?;
                ser.formatter.end_object(&mut ser.writer)
            }
        }
    }

    let ser = MockSer {
        formatter: MockFormatter,
        writer: MockWriter,
    };
    let compound = Compound::Map {
        ser,
        state: State::NonEmpty,
    };

    let _ = end(compound);
}

