// Answer 0

#[test]
fn test_end_with_map_non_empty_state_success() {
    struct MockFormatter {
        called_end_object: bool,
        called_end_object_value: bool,
    }

    impl MockFormatter {
        fn new() -> Self {
            Self {
                called_end_object: false,
                called_end_object_value: false,
            }
        }

        fn end_object(&mut self, _writer: &mut Vec<u8>) -> Result<(), std::io::Error> {
            self.called_end_object = true;
            Ok(())
        }

        fn end_object_value(&mut self, _writer: &mut Vec<u8>) -> Result<(), std::io::Error> {
            self.called_end_object_value = true;
            Ok(())
        }
    }

    struct MockSer {
        formatter: MockFormatter,
        writer: Vec<u8>,
    }

    struct Compound {
        ser: MockSer,
        state: State,
    }

    enum State {
        NonEmpty,
        Empty,
    }

    impl Compound {
        fn end(self) -> Result<(), std::io::Error> {
            match self {
                Compound { ser, state } => {
                    match state {
                        State::Empty => {}
                        _ => ser.formatter.end_object(&mut ser.writer)?,
                    }
                    ser.formatter.end_object_value(&mut ser.writer)?;
                    ser.formatter.end_object(&mut ser.writer)?;
                    Ok(())
                }
            }
        }
    }

    let formatter = MockFormatter::new();
    let writer = Vec::new();
    let ser = MockSer { formatter, writer };
    let compound = Compound { ser, state: State::NonEmpty };

    let result = compound.end();
    
    assert!(result.is_ok());
}

