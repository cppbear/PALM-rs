// Answer 0

#[test]
fn test_serialize_element_map_first_state_error() {
    use serde::ser::{Serialize, Serializer};
    use std::io;

    struct MockWriter {
        should_error: bool,
    }

    impl std::io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
            if self.should_error {
                Err(io::Error::new(io::ErrorKind::Other, "Write Error"))
            } else {
                Ok(buf.len())
            }
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    struct MockFormatter {
        writer: MockWriter,
    }

    impl MockFormatter {
        fn begin_array_value(&mut self, _writer: &mut MockWriter, _is_first: bool) -> io::Result<()> {
            if self.writer.should_error {
                Err(io::Error::new(io::ErrorKind::Other, "Begin Array Value Error"))
            } else {
                Ok(())
            }
        }

        fn end_array_value(&mut self, _writer: &mut MockWriter) -> io::Result<()> {
            Ok(())
        }
    }

    struct MockSerializer {
        formatter: MockFormatter,
    }

    enum State {
        First,
        Rest,
    }

    enum Compound {
        Map {
            ser: MockSerializer,
            state: State,
        },
    }

    struct TestStruct;

    impl Serialize for TestStruct {
        fn serialize<S>(&self, _serializer: S) -> Result<(), S::Error>
        where
            S: Serializer,
        {
            Ok(())
        }
    }

    let mock_writer = MockWriter { should_error: true };
    let mock_formatter = MockFormatter { writer: mock_writer };
    let mock_serializer = MockSerializer { formatter: mock_formatter };
    let mut state = State::First;

    let compound = Compound::Map {
        ser: mock_serializer,
        state,
    };

    if let Compound::Map { ser, state } = compound {
        let result = serialize_element(&mut ser, &TestStruct);
        assert!(result.is_err());
    }
}

