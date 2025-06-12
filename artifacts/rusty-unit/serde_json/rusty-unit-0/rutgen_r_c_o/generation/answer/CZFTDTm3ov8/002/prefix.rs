// Answer 0

#[test]
fn test_serialize_u8_valid_input() {
    let mut writer = Vec::new();
    let formatter = CompactFormatter; 
    let mut serializer = Serializer { writer: &mut writer, formatter };

    let key_serializer = MapKeySerializer { ser: &mut serializer };
    key_serializer.serialize_u8(0); // Testing with the lower bound
    key_serializer.serialize_u8(255); // Testing with the upper bound
}

#[test]
#[should_panic]
fn test_serialize_u8_overflow_input_256() {
    let mut writer = Vec::new();
    let formatter = CompactFormatter; 
    let mut serializer = Serializer { writer: &mut writer, formatter };

    let key_serializer = MapKeySerializer { ser: &mut serializer };
    key_serializer.serialize_u8(256); // Testing with an overflow value
}

#[test]
#[should_panic]
fn test_serialize_u8_overflow_input_257() {
    let mut writer = Vec::new();
    let formatter = CompactFormatter; 
    let mut serializer = Serializer { writer: &mut writer, formatter };

    let key_serializer = MapKeySerializer { ser: &mut serializer };
    key_serializer.serialize_u8(257); // Testing with an overflow value
}

#[test]
fn test_serialize_u8_invalid_write() {
    let mut writer = ErrorWriter; // Custom writer to trigger error
    let formatter = CompactFormatter;
    let mut serializer = Serializer { writer: &mut writer, formatter };

    let key_serializer = MapKeySerializer { ser: &mut serializer };
    let result = key_serializer.serialize_u8(100); // Should return an error due to writer
    drop(result); // To simulate dropping the result without panic
}

struct ErrorWriter;

impl io::Write for ErrorWriter {
    fn write(&mut self, _buf: &[u8]) -> Result<usize> {
        Err(Error); // Force an error on write
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

