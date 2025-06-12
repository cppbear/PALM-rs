// Answer 0

#[test]
fn test_serialize_i8_positive_value() {
    let writer = Vec::new();
    let formatter = CompactFormatter::new();
    let mut serializer = Serializer { writer, formatter };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };
    map_key_serializer.serialize_i8(100);
}

#[test]
fn test_serialize_i8_negative_value() {
    let writer = Vec::new();
    let formatter = CompactFormatter::new();
    let mut serializer = Serializer { writer, formatter };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };
    map_key_serializer.serialize_i8(-50);
}

#[test]
fn test_serialize_i8_zero() {
    let writer = Vec::new();
    let formatter = CompactFormatter::new();
    let mut serializer = Serializer { writer, formatter };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };
    map_key_serializer.serialize_i8(0);
}

#[test]
fn test_serialize_i8_min_value() {
    let writer = Vec::new();
    let formatter = CompactFormatter::new();
    let mut serializer = Serializer { writer, formatter };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };
    map_key_serializer.serialize_i8(-128);
}

#[test]
fn test_serialize_i8_max_value() {
    let writer = Vec::new();
    let formatter = CompactFormatter::new();
    let mut serializer = Serializer { writer, formatter };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };
    map_key_serializer.serialize_i8(127);
}

