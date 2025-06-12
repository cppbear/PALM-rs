// Answer 0

#[test]
fn test_serialize_u64_success() {
    struct FormatterMock;
    struct WriterMock {
        data: Vec<u8>,
    }

    impl FormatterMock {
        fn write_u64(&self, writer: &mut WriterMock, value: u64) -> Result<()> {
            writer.data.extend_from_slice(&value.to_le_bytes());
            Ok(())
        }
    }

    let formatter = FormatterMock;
    let mut writer = WriterMock { data: Vec::new() };
    let result = serialize_u64(formatter, 12345);
    assert!(result.is_ok());
    assert_eq!(writer.data, 12345u64.to_le_bytes());
}

#[test]
#[should_panic]
fn test_serialize_u64_panic_on_writer() {
    struct FaultyFormatterMock;
    struct FaultyWriterMock;

    impl FaultyFormatterMock {
        fn write_u64(&self, _writer: &mut FaultyWriterMock, _value: u64) -> Result<()> {
            panic!("Simulating write failure");
        }
    }

    let formatter = FaultyFormatterMock;
    let mut writer = FaultyWriterMock;
    serialize_u64(formatter, 12345);
}

#[test]
fn test_serialize_u64_large_value() {
    struct FormatterMock;
    struct WriterMock {
        data: Vec<u8>,
    }

    impl FormatterMock {
        fn write_u64(&self, writer: &mut WriterMock, value: u64) -> Result<()> {
            writer.data.extend_from_slice(&value.to_le_bytes());
            Ok(())
        }
    }

    let formatter = FormatterMock;
    let mut writer = WriterMock { data: Vec::new() };
    let result = serialize_u64(formatter, u64::MAX);
    assert!(result.is_ok());
    assert_eq!(writer.data, u64::MAX.to_le_bytes());
}

#[test]
fn test_serialize_u64_zero_value() {
    struct FormatterMock;
    struct WriterMock {
        data: Vec<u8>,
    }

    impl FormatterMock {
        fn write_u64(&self, writer: &mut WriterMock, value: u64) -> Result<()> {
            writer.data.extend_from_slice(&value.to_le_bytes());
            Ok(())
        }
    }

    let formatter = FormatterMock;
    let mut writer = WriterMock { data: Vec::new() };
    let result = serialize_u64(formatter, 0);
    assert!(result.is_ok());
    assert_eq!(writer.data, 0u64.to_le_bytes());
}

