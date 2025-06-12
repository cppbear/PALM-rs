// Answer 0

#[test]
fn test_end_empty_map() {
    struct MockFormatter {
        writer: Vec<u8>,
    }

    impl MockFormatter {
        fn end_array(&self, writer: &mut Vec<u8>) -> Result<(), std::io::Error> {
            writer.push(b']');
            Ok(())
        }
    }

    struct MockSer {
        formatter: MockFormatter,
        writer: Vec<u8>,
    }

    enum State {
        Empty,
        // other states if necessary
    }

    enum Compound {
        Map { ser: MockSer, state: State },
        // other variants if necessary
    }

    let formatter = MockFormatter { writer: vec![] };
    let ser = MockSer { formatter, writer: vec![] };
    let compound = Compound::Map { ser, state: State::Empty };

    let result = match compound {
        Compound::Map { ser, state } => match state {
            State::Empty => Ok(()),
            _ => ser.formatter.end_array(&mut ser.writer).map_err(Error::io),
        },
        #[cfg(feature = "arbitrary_precision")]
        _ => unreachable!(),
        #[cfg(feature = "raw_value")]
        _ => unreachable!(),
    };

    assert!(result.is_ok());
}

#[test]
fn test_end_non_empty_map() {
    struct MockFormatter {
        writer: Vec<u8>,
    }

    impl MockFormatter {
        fn end_array(&self, writer: &mut Vec<u8>) -> Result<(), std::io::Error> {
            writer.push(b']');
            Ok(())
        }
    }

    struct MockSer {
        formatter: MockFormatter,
        writer: Vec<u8>,
    }

    enum State {
        NonEmpty,
        // other states if necessary
    }

    enum Compound {
        Map { ser: MockSer, state: State },
        // other variants if necessary
    }

    let formatter = MockFormatter { writer: vec![] };
    let ser = MockSer { formatter, writer: vec![] };
    let compound = Compound::Map { ser, state: State::NonEmpty };

    let result = match compound {
        Compound::Map { ser, state } => match state {
            State::Empty => Ok(()),
            _ => ser.formatter.end_array(&mut ser.writer).map_err(Error::io),
        },
        #[cfg(feature = "arbitrary_precision")]
        _ => unreachable!(),
        #[cfg(feature = "raw_value")]
        _ => unreachable!(),
    };

    assert!(result.is_ok());
}

#[should_panic]
#[test]
fn test_end_arbitrary_precision() {
    // This test will panic due to the unreachable macro for arbitrary precision.
    #[cfg(feature = "arbitrary_precision")]
    {
        struct MockSer;
        enum Compound {
            Number { ser: MockSer },
            // other variants if necessary
        }
        let compound = Compound::Number { ser: MockSer };
        
        match compound {
            Compound::Number { .. } => unreachable!(),
        }
    }
}

#[should_panic]
#[test]
fn test_end_raw_value() {
    // This test will panic due to the unreachable macro for raw value.
    #[cfg(feature = "raw_value")]
    {
        struct MockSer;
        enum Compound {
            RawValue { ser: MockSer },
            // other variants if necessary
        }
        let compound = Compound::RawValue { ser: MockSer };
        
        match compound {
            Compound::RawValue { .. } => unreachable!(),
        }
    }
}

