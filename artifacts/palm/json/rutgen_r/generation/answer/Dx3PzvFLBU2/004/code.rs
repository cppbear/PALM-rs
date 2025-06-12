// Answer 0

fn serialize_map_test() -> Result<(), Box<dyn std::error::Error>> {
    struct MockFormatter;
    struct MockWriter;

    impl MockFormatter {
        fn begin_object(&self, _writer: &mut MockWriter) -> Result<(), std::io::Error> {
            Ok(())
        }

        fn end_object(&self, _writer: &mut MockWriter) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct Serializer {
        formatter: MockFormatter,
        writer: MockWriter,
    }

    enum State {
        Empty,
        First,
    }

    enum Compound<T> {
        Map { ser: Serializer, state: State },
    }

    impl Serializer {
        fn serialize_map(self, len: Option<usize>) -> Result<Compound<Serializer>, std::io::Error> {
            self.formatter.begin_object(&mut self.writer)?;
            if len == Some(0) {
                self.formatter.end_object(&mut self.writer)?;
                Ok(Compound::Map {
                    ser: self,
                    state: State::Empty,
                })
            } else {
                Ok(Compound::Map {
                    ser: self,
                    state: State::First,
                })
            }
        }
    }

    // Test case where len is not zero
    let serializer = Serializer {
        formatter: MockFormatter,
        writer: MockWriter,
    };

    let result = serializer.serialize_map(Some(1))?;

    match result {
        Compound::Map { ser, state } => {
            assert_eq!(state, State::First);
        }
        _ => panic!("Expected a Compound::Map"),
    }

    Ok(())
}

#[test]
fn test_serialize_map() {
    serialize_map_test().expect("serialize_map failed");
}

