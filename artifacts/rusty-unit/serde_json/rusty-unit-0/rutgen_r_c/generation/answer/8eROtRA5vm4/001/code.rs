// Answer 0

#[test]
fn test_serialize_newtype_struct_valid() {
    use serde::Serialize;

    #[derive(Serialize)]
    struct MyNewtypeStruct(String);

    let my_value = MyNewtypeStruct("test".to_string());

    struct TestWriter;
    impl io::Write for TestWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let mut serializer = Serializer {
        writer: TestWriter,
        formatter: CompactFormatter,
    };

    let result = serializer.serialize_newtype_struct("MyNewtypeStruct", &my_value);
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_serialize_newtype_struct_panics_on_invalid() {
    use serde::Serialize;

    #[derive(Serialize)]
    struct InvalidStruct;

    let my_value = InvalidStruct;

    struct TestWriter;
    impl io::Write for TestWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let mut serializer = Serializer {
        writer: TestWriter,
        formatter: CompactFormatter,
    };

    serializer.serialize_newtype_struct("InvalidStruct", &my_value).unwrap();
}

#[test]
fn test_serialize_newtype_struct_empty() {
    use serde::Serialize;

    #[derive(Serialize)]
    struct EmptyNewtypeStruct;

    let my_value = EmptyNewtypeStruct;

    struct TestWriter;
    impl io::Write for TestWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let mut serializer = Serializer {
        writer: TestWriter,
        formatter: CompactFormatter,
    };

    let result = serializer.serialize_newtype_struct("EmptyNewtypeStruct", &my_value);
    assert!(result.is_ok());
}

