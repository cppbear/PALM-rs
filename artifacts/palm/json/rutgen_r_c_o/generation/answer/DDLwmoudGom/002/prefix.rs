// Answer 0

#[test]
fn test_serialize_u16_valid_value() {
    let value: u16 = 12345;
    let mut writer = Vec::new();
    let formatter = CompactFormatter {};
    let serializer = Serializer { writer, formatter };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };
    map_key_serializer.serialize_u16(value);
}

#[test]
fn test_serialize_u16_zero() {
    let value: u16 = 0;
    let mut writer = Vec::new();
    let formatter = CompactFormatter {};
    let serializer = Serializer { writer, formatter };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };
    map_key_serializer.serialize_u16(value);
}

#[test]
fn test_serialize_u16_max_value() {
    let value: u16 = 65535;
    let mut writer = Vec::new();
    let formatter = CompactFormatter {};
    let serializer = Serializer { writer, formatter };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };
    map_key_serializer.serialize_u16(value);
}

#[should_panic]
#[test]
fn test_serialize_u16_io_error() {
    let value: u16 = 32800; // Arbitrary value, related to failure in writing
    let mut faulty_writer = FaultyWriter {};
    let formatter = CompactFormatter {};
    let serializer = Serializer { writer: faulty_writer, formatter };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };
    map_key_serializer.serialize_u16(value);
}

struct FaultyWriter;

impl io::Write for FaultyWriter {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        Err(Error::from(ErrorCode::Io))
    }
    
    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

