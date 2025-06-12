// Answer 0

#[test]
fn test_serialize_bytes_should_return_error() {
    struct TestWriter;

    impl io::Write for TestWriter {
        fn write(&mut self, _: &[u8]) -> io::Result<usize> {
            Ok(0)
        }
        
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = TestWriter;
    let serializer = MapKeySerializer { ser: &mut Serializer { writer, formatter: () } };

    let result = serializer.serialize_bytes(&[1, 2, 3]);
    assert!(result.is_err());
}

#[test]
fn test_serialize_bytes_should_return_specific_error() {
    struct TestWriter;

    impl io::Write for TestWriter {
        fn write(&mut self, _: &[u8]) -> io::Result<usize> {
            Ok(0)
        }
        
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = TestWriter;
    let serializer = MapKeySerializer { ser: &mut Serializer { writer, formatter: () } };

    let result = serializer.serialize_bytes(&[1, 2, 3]);
    if let Err(e) = result {
        assert_eq!(e, key_must_be_a_string());
    } else {
        panic!("Expected an error but got Ok")
    }
}

