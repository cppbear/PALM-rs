// Answer 0

#[test]
fn test_serialize_map_non_empty() {
    struct TestFormatter;

    impl TestFormatter {
        fn begin_object(&mut self, _writer: &mut ()) -> Result<(), ()> {
            Ok(())
        }

        fn end_object(&mut self, _writer: &mut ()) -> Result<(), ()> {
            Ok(())
        }
    }

    struct TestSerializer {
        formatter: TestFormatter,
        writer: (),
    }

    impl TestSerializer {
        fn serialize_map(self, len: Option<usize>) -> Result<Compound<TestSerializer>> {
            tri!(self
                .formatter
                .begin_object(&mut self.writer)
                .map_err(Error::io));
            if len == Some(0) {
                tri!(self
                    .formatter
                    .end_object(&mut self.writer)
                    .map_err(Error::io));
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

    #[derive(Debug, PartialEq)]
    struct Compound<S> {
        ser: S,
        state: State,
    }

    #[derive(Debug, PartialEq)]
    enum State {
        Empty,
        First,
    }

    #[derive(Debug)]
    enum Error {
        Io,
    }

    let serializer = TestSerializer {
        formatter: TestFormatter,
        writer: (),
    };

    let result = serializer.serialize_map(Some(1));
    assert!(result.is_ok());
    if let Ok(Compound { ser, state }) = result {
        assert_eq!(state, State::First);
        assert_eq!(ser.formatter, TestFormatter);
    }
}

