// Answer 0

#[test]
fn test_end_with_err_on_formatter_end_object() {
    struct MockFormatter;
    struct MockWriter;

    impl MockFormatter {
        fn end_object(&self, _writer: &mut MockWriter) -> Result<(), std::io::Error> {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "mock error"))
        }

        fn end_object_value(&self, _writer: &mut MockWriter) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct MockSer {
        formatter: MockFormatter,
        writer: MockWriter,
    }

    enum State {
        Empty,
        Other,
    }

    enum Compound {
        Map { ser: MockSer, state: State },
    }

    impl Compound {
        fn end(self) -> Result<(), std::io::Error> {
            match self {
                Compound::Map { ser, state } => {
                    match state {
                        State::Empty => {}
                        _ => ser.formatter.end_object(&mut ser.writer)?,
                    }
                    ser.formatter.end_object_value(&mut ser.writer)?;
                    ser.formatter.end_object(&mut ser.writer)
                }
            }
        }
    }

    let mock_ser = MockSer {
        formatter: MockFormatter,
        writer: MockWriter,
    };
    
    let compound = Compound::Map {
        ser: mock_ser,
        state: State::Other,
    };

    let result = compound.end();
    assert!(result.is_err());
}

