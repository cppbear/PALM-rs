// Answer 0

#[test]
fn test_serialize_bool_true_ok() {
    let writer = Vec::new();
    let formatter = CompactFormatter::default();
    let mut serializer = Serializer {
        writer,
        formatter,
    };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };
    map_key_serializer.serialize_bool(true);
}

#[test]
fn test_serialize_bool_false_ok() {
    let writer = Vec::new();
    let formatter = CompactFormatter::default();
    let mut serializer = Serializer {
        writer,
        formatter,
    };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };
    map_key_serializer.serialize_bool(false);
}

#[should_panic]
fn test_serialize_bool_err() {
    let writer = MockWriter { should_fail: true };
    let formatter = CompactFormatter::default();
    let mut serializer = Serializer {
        writer,
        formatter,
    };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };
    map_key_serializer.serialize_bool(true);
}

#[should_panic]
fn test_serialize_bool_err_false() {
    let writer = MockWriter { should_fail: true };
    let formatter = CompactFormatter::default();
    let mut serializer = Serializer {
        writer,
        formatter,
    };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };
    map_key_serializer.serialize_bool(false);
}

struct MockWriter {
    should_fail: bool,
}

impl io::Write for MockWriter {
    fn write(&mut self, _: &[u8]) -> std::io::Result<usize> {
        if self.should_fail {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "Mock failure"))
        } else {
            Ok(1)
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

