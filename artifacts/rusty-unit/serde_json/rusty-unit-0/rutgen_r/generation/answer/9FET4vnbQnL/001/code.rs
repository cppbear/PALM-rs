// Answer 0

#[test]
fn test_serialize_i32_positive_value() {
    struct DummyWriter {
        output: Vec<u8>,
    }

    impl DummyWriter {
        fn new() -> Self {
            DummyWriter { output: Vec::new() }
        }

        fn write_i32(&mut self, _: &mut Self, value: i32) -> Result<(), std::io::Error> {
            self.output.extend_from_slice(&value.to_le_bytes());
            Ok(())
        }
    }

    struct Serializer<'a> {
        writer: &'a mut DummyWriter,
        formatter: &'a mut DummyWriter,
    }

    let mut writer = DummyWriter::new();
    let mut formatter = DummyWriter::new();
    let mut serializer = Serializer {
        writer: &mut writer,
        formatter: &mut formatter,
    };
    
    let result = serializer.serialize_i32(42);
    assert!(result.is_ok());
}

#[test]
fn test_serialize_i32_negative_value() {
    struct DummyWriter {
        output: Vec<u8>,
    }

    impl DummyWriter {
        fn new() -> Self {
            DummyWriter { output: Vec::new() }
        }

        fn write_i32(&mut self, _: &mut Self, value: i32) -> Result<(), std::io::Error> {
            self.output.extend_from_slice(&value.to_le_bytes());
            Ok(())
        }
    }

    struct Serializer<'a> {
        writer: &'a mut DummyWriter,
        formatter: &'a mut DummyWriter,
    }

    let mut writer = DummyWriter::new();
    let mut formatter = DummyWriter::new();
    let mut serializer = Serializer {
        writer: &mut writer,
        formatter: &mut formatter,
    };

    let result = serializer.serialize_i32(-42);
    assert!(result.is_ok());
}

#[test]
fn test_serialize_i32_zero_value() {
    struct DummyWriter {
        output: Vec<u8>,
    }

    impl DummyWriter {
        fn new() -> Self {
            DummyWriter { output: Vec::new() }
        }

        fn write_i32(&mut self, _: &mut Self, value: i32) -> Result<(), std::io::Error> {
            self.output.extend_from_slice(&value.to_le_bytes());
            Ok(())
        }
    }

    struct Serializer<'a> {
        writer: &'a mut DummyWriter,
        formatter: &'a mut DummyWriter,
    }

    let mut writer = DummyWriter::new();
    let mut formatter = DummyWriter::new();
    let mut serializer = Serializer {
        writer: &mut writer,
        formatter: &mut formatter,
    };

    let result = serializer.serialize_i32(0);
    assert!(result.is_ok());
}

