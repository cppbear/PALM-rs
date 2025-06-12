// Answer 0

fn test_end_with_empty_state() -> Result<()> {
    struct MockWriter;

    impl MockWriter {
        fn new() -> Self {
            MockWriter {}
        }
    }

    struct MockFormatter;

    impl MockFormatter {
        fn end_array(&self, _writer: &mut MockWriter) -> Result<()> {
            Ok(())
        }

        fn end_object_value(&self, _writer: &mut MockWriter) -> Result<()> {
            Ok(())
        }

        fn end_object(&self, _writer: &mut MockWriter) -> Result<()> {
            Ok(())
        }
    }

    struct MockSer {
        formatter: MockFormatter,
        writer: MockWriter,
    }

    enum State {
        Empty,
        NotEmpty,
    }

    enum Compound {
        Map {
            ser: MockSer,
            state: State,
        },
    }

    impl Compound {
        fn end(self) -> Result<()> {
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
    }

    let writer = MockWriter::new();
    let formatter = MockFormatter {};
    let ser = MockSer {
        formatter,
        writer,
    };
    let compound = Compound::Map {
        ser,
        state: State::Empty,
    };

    compound.end()?;
    Ok(())
}

#[test]
fn test_end() {
    let result = test_end_with_empty_state();
    assert!(result.is_ok());
}

