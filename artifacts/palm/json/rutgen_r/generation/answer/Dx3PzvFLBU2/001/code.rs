// Answer 0

#[test]
fn test_serialize_map_begin_object_error() {
    struct MockFormatter;

    impl MockFormatter {
        fn begin_object(&self, _: &mut ()) -> Result<(), String> {
            Err("IO error".to_string())
        }

        fn end_object(&self, _: &mut ()) -> Result<(), String> {
            Ok(())
        }
    }

    struct MockSerializer {
        formatter: MockFormatter,
        writer: (),
    }

    enum State {
        Empty,
        First,
    }

    enum Compound {
        Map { ser: MockSerializer, state: State },
    }

    impl MockSerializer {
        fn serialize_map(self, len: Option<usize>) -> Result<Compound, String> {
            self.formatter
                .begin_object(&mut self.writer)
                .map_err(|e| e)?;

            if len == Some(0) {
                self.formatter
                    .end_object(&mut self.writer)
                    .map_err(|e| e)?;
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

    let serializer = MockSerializer {
        formatter: MockFormatter,
        writer: (),
    };

    let result = serializer.serialize_map(None);
    assert_eq!(result, Err("IO error".to_string()));
}

#[test]
fn test_serialize_map_empty_error() {
    struct MockFormatter;

    impl MockFormatter {
        fn begin_object(&self, _: &mut ()) -> Result<(), String> {
            Ok(())
        }

        fn end_object(&self, _: &mut ()) -> Result<(), String> {
            Err("IO error".to_string())
        }
    }

    struct MockSerializer {
        formatter: MockFormatter,
        writer: (),
    }

    enum State {
        Empty,
        First,
    }

    enum Compound {
        Map { ser: MockSerializer, state: State },
    }

    impl MockSerializer {
        fn serialize_map(self, len: Option<usize>) -> Result<Compound, String> {
            self.formatter
                .begin_object(&mut self.writer)
                .map_err(|e| e)?;

            if len == Some(0) {
                self.formatter
                    .end_object(&mut self.writer)
                    .map_err(|e| e)?;
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

    let serializer = MockSerializer {
        formatter: MockFormatter,
        writer: (),
    };

    let result = serializer.serialize_map(Some(0));
    assert_eq!(result, Err("IO error".to_string()));
}

