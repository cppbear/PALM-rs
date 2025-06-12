// Answer 0

#[test]
fn test_serialize_char_valid() {
    struct MockWriter;
    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    let mut writer = MockWriter;
    let formatter = MockFormatter;

    let serializer = Serializer {
        writer,
        formatter,
    };

    let result = serializer.serialize_char('a');
    assert!(result.is_ok());

    let result = serializer.serialize_char('A');
    assert!(result.is_ok());

    let result = serializer.serialize_char('1');
    assert!(result.is_ok());

    let result = serializer.serialize_char('ñ');
    assert!(result.is_ok());

    let result = serializer.serialize_char('中'); // Testing non-ASCII character
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_serialize_char_invalid() {
    struct MockWriter;
    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            panic!("Mock panic");
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    let mut writer = MockWriter;
    let formatter = MockFormatter;

    let serializer = Serializer {
        writer,
        formatter,
    };

    let _result = serializer.serialize_char('X'); // This will trigger panic in writer
}

