// Answer 0

#[test]
fn test_serialize_key_with_begin_object_key_error() {
    use serde::Serialize;
    use serde_json::ser::{Serializer, State, Compound, Error};
    use std::io::{self, Write};

    struct FaultyWriter;
    impl Write for FaultyWriter {
        fn write(&mut self, _: &[u8]) -> io::Result<usize> {
            Err(io::Error::new(io::ErrorKind::Other, "write error"))
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    struct FakeFormatter {
        writer: FaultyWriter,
    }

    impl FakeFormatter {
        fn begin_object_key(&mut self, _: &mut FaultyWriter, _: bool) -> Result<(), Error> {
            Err(Error::io(io::Error::new(io::ErrorKind::Other, "formatter error")))
        }

        fn end_object_key(&mut self, _: &mut FaultyWriter) -> Result<(), Error> {
            Ok(())
        }
    }

    struct FakeSerializer {
        formatter: FakeFormatter,
        state: State,
    }

    impl FakeSerializer {
        fn new() -> Self {
            FakeSerializer {
                formatter: FakeFormatter {
                    writer: FaultyWriter,
                },
                state: State::First,
            }
        } 
    }

    struct MapKeySerializer<'a> {
        ser: &'a mut FakeSerializer,
    }

    impl<'a> Serialize for MapKeySerializer<'a> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serializer.serialize_str("key")
        }
    }

    let mut serializer = FakeSerializer::new();
    let key = MapKeySerializer { ser: &mut serializer };

    let result = serializer.serialize_key(&key);
    assert!(result.is_err());
}

