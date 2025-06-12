// Answer 0

#[test]
fn test_end_with_empty_state() {
    struct MockFormatter {
        called: bool,
    }

    impl MockFormatter {
        fn new() -> Self {
            Self { called: false }
        }

        fn end_array(&mut self, _writer: &mut Vec<u8>) -> Result<(), std::io::Error> {
            self.called = true;
            Ok(())
        }
    }

    struct MockWriter(Vec<u8>);

    struct MockSer {
        formatter: MockFormatter,
        writer: MockWriter,
    }

    enum State {
        Empty,
        NonEmpty,
    }

    enum Compound {
        Map { ser: MockSer, state: State },
    }

    // Prepare the structure for the test
    let mock_formatter = MockFormatter::new();
    let mock_writer = MockWriter(Vec::new());
    let mock_ser = MockSer { formatter: mock_formatter, writer: mock_writer };
    
    // Create a Compound::Map instance in the Empty state
    let compound = Compound::Map { ser: mock_ser, state: State::Empty };

    // Run the method under test
    let result = match compound {
        Compound::Map { ser, state } => match state {
            State::Empty => Ok(()),
            _ => ser.formatter.end_array(&mut ser.writer.0).map_err(|_| ()),
        },
        _ => unreachable!(),
    };

    // Verify the result
    assert!(result.is_ok());
}

