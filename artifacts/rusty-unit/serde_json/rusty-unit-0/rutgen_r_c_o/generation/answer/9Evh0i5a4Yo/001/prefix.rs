// Answer 0

#[test]
fn test_serialize_bool_true() {
    struct MockWriter;
    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> { Ok(0) }
        fn write_all(&mut self, _buf: &[u8]) -> Result<()> { Ok(()) }
        fn flush(&mut self) -> Result<()> { Ok(()) }
    }
    struct MockFormatter;
    impl Formatter for MockFormatter {
        fn write_bool(&self, _writer: &mut dyn io::Write, _value: bool) -> Result<()> { Ok(()) }
    }
    
    let mut writer = MockWriter;
    let formatter = MockFormatter;
    let serializer = &mut Serializer { writer, formatter };
    serializer.serialize_bool(true);
}

#[test]
fn test_serialize_bool_false() {
    struct MockWriter;
    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> { Ok(0) }
        fn write_all(&mut self, _buf: &[u8]) -> Result<()> { Ok(()) }
        fn flush(&mut self) -> Result<()> { Ok(()) }
    }
    struct MockFormatter;
    impl Formatter for MockFormatter {
        fn write_bool(&self, _writer: &mut dyn io::Write, _value: bool) -> Result<()> { Ok(()) }
    }
    
    let mut writer = MockWriter;
    let formatter = MockFormatter;
    let serializer = &mut Serializer { writer, formatter };
    serializer.serialize_bool(false);
}

