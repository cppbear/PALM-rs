// Answer 0

#[test]
fn test_serialize_seq_non_empty() {
    struct MockFormatter;
    struct MockWriter;

    impl MockFormatter {
        fn begin_array(&self, _writer: &mut MockWriter) -> Result<(), std::io::Error> {
            Ok(())
        }
        fn end_array(&self, _writer: &mut MockWriter) -> Result<(), std::io::Error> {
            // This method won't be called in this test
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
        fn serialize_seq(self, len: Option<usize>) -> Result<Compound, std::io::Error> {
            self.formatter.begin_array(&mut self.writer)?;
            if len == Some(0) {
                self.formatter.end_array(&mut self.writer)?;
                return Ok(Compound::Map {
                    ser: self,
                    state: State::Empty,
                });
            }
            Ok(Compound::Map {
                ser: self,
                state: State::First,
            })
        }
    }

    let serializer = Serializer {
        formatter: MockFormatter,
        writer: MockWriter,
    };

    let result = serializer.serialize_seq(Some(1)).unwrap();
    match result {
        Compound::Map { ser, state } if state == State::First => {
            // Successful test; the return state is as expected.
        },
        _ => panic!("Test failed: expected state to be State::First"),
    }
}

