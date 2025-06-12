// Answer 0

#[test]
#[should_panic]
fn test_serialize_seq_begin_array_error() {
    struct MockWriter;
    struct MockFormatter;

    impl MockFormatter {
        fn begin_array(&mut self, _writer: &mut MockWriter) -> Result<(), String> {
            Err(String::from("begin_array error"))
        }

        fn end_array(&mut self, _writer: &mut MockWriter) -> Result<(), String> {
            Ok(())
        }
    }

    struct Serializer {
        writer: MockWriter,
        formatter: MockFormatter,
    }

    enum State {
        Empty,
        First,
    }

    enum Compound {
        Map {
            ser: Serializer,
            state: State,
        },
    }

    impl Serializer {
        fn serialize_seq(self, len: Option<usize>) -> Result<Compound, String> {
            self.formatter.begin_array(&mut self.writer).map_err(|e| e.to_string())?;
            if len == Some(0) {
                self.formatter.end_array(&mut self.writer).map_err(|e| e.to_string())?;
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
        writer: MockWriter,
        formatter: MockFormatter,
    };

    let _result = serializer.serialize_seq(Some(1)).unwrap();
}

