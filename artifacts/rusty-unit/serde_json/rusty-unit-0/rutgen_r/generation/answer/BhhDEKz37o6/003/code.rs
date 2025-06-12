// Answer 0

fn test_end_function() {
    struct MockWriter;
    struct MockFormatter;
    
    impl MockFormatter {
        fn end_object(&self, _: &mut MockWriter) -> Result<(), std::io::Error> {
            Ok(())
        }
        
        fn end_object_value(&self, _: &mut MockWriter) -> Result<(), std::io::Error> {
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

    let mock_writer = MockWriter;
    let mock_formatter = MockFormatter;
    let mock_ser = MockSer { formatter: mock_formatter, writer: mock_writer };

    let compound = Compound::Map { ser: mock_ser, state: State::NotEmpty };
    
    let result = compound.end();
    assert!(result.is_ok());
}

