// Answer 0

#[test]
fn test_serialize_i64_ok_positive() {
    let mut writer = Vec::new();
    let formatter = CompactFormatter;
    let mut serializer = Serializer { writer, formatter };
    let result = serializer.serialize_i64(42);
}

#[test]
fn test_serialize_i64_ok_negative() {
    let mut writer = Vec::new();
    let formatter = CompactFormatter;
    let mut serializer = Serializer { writer, formatter };
    let result = serializer.serialize_i64(-42);
}

#[test]
fn test_serialize_i64_ok_zero() {
    let mut writer = Vec::new();
    let formatter = CompactFormatter;
    let mut serializer = Serializer { writer, formatter };
    let result = serializer.serialize_i64(0);
}

#[test]
fn test_serialize_i64_min() {
    let mut writer = Vec::new();
    let formatter = CompactFormatter;
    let mut serializer = Serializer { writer, formatter };
    let result = serializer.serialize_i64(i64::MIN);
}

#[test]
fn test_serialize_i64_max() {
    let mut writer = Vec::new();
    let formatter = CompactFormatter;
    let mut serializer = Serializer { writer, formatter };
    let result = serializer.serialize_i64(i64::MAX);
}

#[test]
#[should_panic]
fn test_serialize_i64_err_trigger() {
    let mut writer = ErrWriter; // Assumes ErrWriter simulates an error
    let formatter = CompactFormatter;
    let mut serializer = Serializer { writer, formatter };
    let result = serializer.serialize_i64(100);
}

struct ErrWriter;

impl io::Write for ErrWriter {
    fn write(&mut self, _: &[u8]) -> Result<usize> {
        Err(Error::io(std::io::Error::new(std::io::ErrorKind::Other, "write error")))
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

