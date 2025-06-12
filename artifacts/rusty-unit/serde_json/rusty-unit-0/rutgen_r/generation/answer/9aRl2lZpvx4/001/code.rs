// Answer 0

#[test]
fn test_end_with_empty_state() {
    struct MockWriter;
    struct MockFormatter {
        writer: MockWriter,
    }
    
    struct MockSer {
        formatter: MockFormatter,
        state: State,
    }

    impl MockSer {
        fn end_object(&mut self, _writer: &mut MockWriter) -> Result<()> {
            Ok(())
        }
    }

    let writer = MockWriter;
    let formatter = MockFormatter { writer };
    let ser = MockSer { formatter, state: State::NonEmpty };

    let result = Compound::Map { ser, state: State::NonEmpty }.end();
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_end_with_unreachable_number() {
    struct MockWriter;
    struct MockFormatter {
        writer: MockWriter,
    }

    struct MockSer {
        formatter: MockFormatter,
        state: State,
    }

    impl MockSer {
        fn end_object(&mut self, _writer: &mut MockWriter) -> Result<()> {
            unreachable!();
        }
    }

    let writer = MockWriter;
    let formatter = MockFormatter { writer };

    let ser = MockSer { formatter, state: State::NonEmpty };

    let _ = Compound::Number { /* fields */ }.end(); // Will panic as expected
}

#[test]
#[should_panic]
fn test_end_with_unreachable_raw_value() {
    struct MockWriter;
    struct MockFormatter {
        writer: MockWriter,
    }

    struct MockSer {
        formatter: MockFormatter,
        state: State,
    }

    impl MockSer {
        fn end_object(&mut self, _writer: &mut MockWriter) -> Result<()> {
            unreachable!();
        }
    }

    let writer = MockWriter;
    let formatter = MockFormatter { writer };

    let ser = MockSer { formatter, state: State::NonEmpty };

    let _ = Compound::RawValue { /* fields */ }.end(); // Will panic as expected
}

