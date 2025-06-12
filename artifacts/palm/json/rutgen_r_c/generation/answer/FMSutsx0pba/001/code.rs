// Answer 0

#[test]
fn test_end_with_map_variant() {
    struct MockWriter;
    impl io::Write for MockWriter {
        fn write(&mut self, _: &[u8]) -> core::result::Result<usize, io::Error> {
            Ok(0)
        }
        fn flush(&mut self) -> core::result::Result<(), io::Error> {
            Ok(())
        }
    }

    struct MockFormatter;

    let mut serializer = Serializer {
        writer: MockWriter,
        formatter: MockFormatter,
    };

    let map = Map::<String, Value>::new();
    let mut compound = Compound::Map { ser: &mut serializer, state: State::Empty };

    assert_eq!(compound.end(), Ok(()));
}

#[test]
#[cfg(feature = "arbitrary_precision")]
fn test_end_with_number_variant() {
    struct MockWriter;
    impl io::Write for MockWriter {
        fn write(&mut self, _: &[u8]) -> core::result::Result<usize, io::Error> {
            Ok(0)
        }
        fn flush(&mut self) -> core::result::Result<(), io::Error> {
            Ok(())
        }
    }

    struct MockFormatter;

    let mut serializer = Serializer {
        writer: MockWriter,
        formatter: MockFormatter,
    };

    let mut compound = Compound::Number { ser: &mut serializer };

    assert_eq!(compound.end(), Ok(()));
}

#[test]
#[cfg(feature = "raw_value")]
fn test_end_with_raw_value_variant() {
    struct MockWriter;
    impl io::Write for MockWriter {
        fn write(&mut self, _: &[u8]) -> core::result::Result<usize, io::Error> {
            Ok(0)
        }
        fn flush(&mut self) -> core::result::Result<(), io::Error> {
            Ok(())
        }
    }

    struct MockFormatter;

    let mut serializer = Serializer {
        writer: MockWriter,
        formatter: MockFormatter,
    };

    let mut compound = Compound::RawValue { ser: &mut serializer };

    assert_eq!(compound.end(), Ok(()));
}

