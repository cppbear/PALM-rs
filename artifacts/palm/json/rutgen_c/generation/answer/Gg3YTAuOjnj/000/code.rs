// Answer 0

#[test]
fn test_serialize_str_empty() {
    struct MockWriter;
    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn write_all(&mut self, buf: &[u8]) -> Result<()> {
            Ok(())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;
    
    impl Formatter for MockFormatter {}

    let mut writer = MockWriter;
    let formatter = MockFormatter;
    let serializer = Serializer {
        writer: &mut writer,
        formatter,
    };
    
    let result = serializer.serialize_str("");
    assert!(result.is_ok());
}

#[test]
fn test_serialize_str_normal() {
    struct MockWriter;
    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn write_all(&mut self, buf: &[u8]) -> Result<()> {
            Ok(())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;
    
    impl Formatter for MockFormatter {}

    let mut writer = MockWriter;
    let formatter = MockFormatter;
    let serializer = Serializer {
        writer: &mut writer,
        formatter,
    };
    
    let result = serializer.serialize_str("hello");
    assert!(result.is_ok());
}

#[test]
fn test_serialize_str_escaped() {
    struct MockWriter;
    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn write_all(&mut self, buf: &[u8]) -> Result<()> {
            Ok(())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;
    
    impl Formatter for MockFormatter {}

    let mut writer = MockWriter;
    let formatter = MockFormatter;
    let serializer = Serializer {
        writer: &mut writer,
        formatter,
    };

    let result = serializer.serialize_str("hello \"world\"");
    assert!(result.is_ok());
}

