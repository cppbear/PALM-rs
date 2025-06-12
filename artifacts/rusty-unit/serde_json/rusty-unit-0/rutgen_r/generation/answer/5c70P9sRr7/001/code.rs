// Answer 0

fn test_end_function_map_state_not_empty_end_array_err() {
    struct MockFormatter {
        writer: Vec<u8>,
    }

    impl MockFormatter {
        fn end_array(&mut self, _writer: &mut Vec<u8>) -> Result<()> {
            Err(Error::io)
        }

        fn end_object_value(&mut self, _writer: &mut Vec<u8>) -> Result<()> {
            Ok(())
        }

        fn end_object(&mut self, _writer: &mut Vec<u8>) -> Result<()> {
            Ok(())
        }
    }

    struct MockSer {
        formatter: MockFormatter,
        writer: Vec<u8>,
    }

    enum State {
        Empty,
        NotEmpty,
    }

    enum Compound {
        Map { ser: MockSer, state: State },
        #[cfg(feature = "arbitrary_precision")]
        Number { },
        #[cfg(feature = "raw_value")]
        RawValue { },
    }

    impl Compound {
        fn end(self) -> Result<()> {
            match self {
                Compound::Map { ser, state } => {
                    match state {
                        State::Empty => {}
                        _ => tri!(ser.formatter.end_array(&mut ser.writer).map_err(Error::io)),
                    }
                    tri!(ser
                        .formatter
                        .end_object_value(&mut ser.writer)
                        .map_err(Error::io));
                    ser.formatter.end_object(&mut ser.writer).map_err(Error::io)
                }
                #[cfg(feature = "arbitrary_precision")]
                Compound::Number { .. } => unreachable!(),
                #[cfg(feature = "raw_value")]
                Compound::RawValue { .. } => unreachable!(),
            }
        }
    }

    let mock_formatter = MockFormatter { writer: vec![] };
    let mock_ser = MockSer { formatter: mock_formatter, writer: vec![] };
    let compound = Compound::Map { ser: mock_ser, state: State::NotEmpty };

    let result = compound.end();
    
    assert!(result.is_err());
}

