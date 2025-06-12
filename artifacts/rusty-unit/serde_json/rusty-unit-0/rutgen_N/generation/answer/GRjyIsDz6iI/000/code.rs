// Answer 0

#[test]
fn test_serialize_u64_success() {
    struct TestFormatter;
    struct TestWriter {
        output: Vec<u8>,
    }
    
    impl TestWriter {
        fn new() -> Self {
            Self { output: Vec::new() }
        }
    }
    
    impl TestFormatter {
        fn write_u64(&self, writer: &mut TestWriter, value: u64) -> Result<()> {
            write!(writer.output, "{}", value).map_err(|_| Error::io())
        }
    }
    
    struct TestSerializer {
        formatter: TestFormatter,
        writer: TestWriter,
    }
    
    impl TestSerializer {
        fn new(formatter: TestFormatter, writer: TestWriter) -> Self {
            Self { formatter, writer }
        }
        
        fn serialize_u64(self, value: u64) -> Result<()> {
            self.formatter.write_u64(&mut self.writer, value)
                .map_err(Error::io)
        }
    }
    
    let formatter = TestFormatter;
    let writer = TestWriter::new();
    let serializer = TestSerializer::new(formatter, writer);
    
    let result = serializer.serialize_u64(42);
    
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_serialize_u64_failure() {
    struct FaultyFormatter;
    struct TestWriter {
        output: Vec<u8>,
    }
    
    impl TestWriter {
        fn new() -> Self {
            Self { output: Vec::new() }
        }
    }
    
    impl FaultyFormatter {
        fn write_u64(&self, writer: &mut TestWriter, _value: u64) -> Result<()> {
            panic!("I/O Error");
        }
    }
    
    struct FaultySerializer {
        formatter: FaultyFormatter,
        writer: TestWriter,
    }
    
    impl FaultySerializer {
        fn new(formatter: FaultyFormatter, writer: TestWriter) -> Self {
            Self { formatter, writer }
        }
        
        fn serialize_u64(self, value: u64) -> Result<()> {
            self.formatter.write_u64(&mut self.writer, value)
                .map_err(Error::io)
        }
    }
    
    let formatter = FaultyFormatter;
    let writer = TestWriter::new();
    let serializer = FaultySerializer::new(formatter, writer);
    
    serializer.serialize_u64(42).unwrap();
}

