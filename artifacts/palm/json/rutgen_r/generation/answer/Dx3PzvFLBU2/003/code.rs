// Answer 0

#[test]
fn test_serialize_map_empty() {
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

    enum Compound {
        Map { ser: Serializer, state: State },
    }

    impl Serializer {
        fn serialize_map(self, len: Option<usize>) -> Result<Compound, std::io::Error> {
            if self.formatter.begin_object(&mut self.writer).is_err() {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "begin_object failed"));
            }

            if len == Some(0) {
                if self.formatter.end_object(&mut self.writer).is_err() {
                    return Err(std::io::Error::new(std::io::ErrorKind::Other, "end_object failed"));
                }
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

    let serializer = Serializer {
        formatter: MockFormatter,
        writer: MockWriter,
    };

    let result = serializer.serialize_map(Some(0)).unwrap();
    match result {
        Compound::Map { ser, state } => {
            assert_eq!(state, State::Empty);
        },
    }
}

