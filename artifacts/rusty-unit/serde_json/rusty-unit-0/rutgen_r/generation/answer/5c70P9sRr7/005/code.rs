// Answer 0

#[test]
fn test_end_with_empty_state() {
    struct MockFormatter {
        end_object_value_called: bool,
        end_object_called: bool,
    }
    
    impl MockFormatter {
        fn new() -> Self {
            MockFormatter {
                end_object_value_called: false,
                end_object_called: false,
            }
        }

        fn end_object_value(&mut self, _: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
            self.end_object_value_called = true;
            Ok(())
        }

        fn end_object(&mut self, _: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
            self.end_object_called = true;
            Ok(())
        }
        
        fn end_array(&mut self, _: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct MockWriter;

    struct MockSer {
        writer: MockWriter,
        formatter: MockFormatter,
    }

    enum State {
        Empty,
        NonEmpty,
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
                        _ => ser.formatter.end_array(&mut ser.writer)?,
                    }
                    ser.formatter.end_object_value(&mut ser.writer)?;
                    ser.formatter.end_object(&mut ser.writer)
                }
            }
        }
    }

    let formatter = MockFormatter::new();
    let writer = MockWriter;
    let ser = MockSer { writer, formatter };
    let compound = Compound::Map { ser, state: State::Empty };

    let result = compound.end();
    
    assert!(result.is_ok());
    assert!(matches!(compound, Compound::Map { ser, state: State::Empty }));
    if let Compound::Map { ser, .. } = compound {
        assert!(ser.formatter.end_object_value_called);
        assert!(ser.formatter.end_object_called);
    }
}

