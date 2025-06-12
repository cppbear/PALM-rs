// Answer 0

#[test]
fn test_serialize_f64_zero() {
    let mut writer = Vec::new();
    let formatter = CompactFormatter::new();
    let serializer = Serializer { writer, formatter };
    let mut map_key_serializer = MapKeySerializer { ser: &mut serializer };
    map_key_serializer.serialize_f64(0.0);
}

#[test]
fn test_serialize_f64_negative() {
    let mut writer = Vec::new();
    let formatter = CompactFormatter::new();
    let serializer = Serializer { writer, formatter };
    let mut map_key_serializer = MapKeySerializer { ser: &mut serializer };
    map_key_serializer.serialize_f64(-1.0);
}

#[test]
fn test_serialize_f64_positive() {
    let mut writer = Vec::new();
    let formatter = CompactFormatter::new();
    let serializer = Serializer { writer, formatter };
    let mut map_key_serializer = MapKeySerializer { ser: &mut serializer };
    map_key_serializer.serialize_f64(1.0);
}

#[test]
fn test_serialize_f64_large_positive() {
    let mut writer = Vec::new();
    let formatter = CompactFormatter::new();
    let serializer = Serializer { writer, formatter };
    let mut map_key_serializer = MapKeySerializer { ser: &mut serializer };
    map_key_serializer.serialize_f64(1.7976931348623157e+308);
}

#[test]
fn test_serialize_f64_small_positive() {
    let mut writer = Vec::new();
    let formatter = CompactFormatter::new();
    let serializer = Serializer { writer, formatter };
    let mut map_key_serializer = MapKeySerializer { ser: &mut serializer };
    map_key_serializer.serialize_f64(2.220446049250313e-16);
}

#[test]
fn test_serialize_f64_pi() {
    let mut writer = Vec::new();
    let formatter = CompactFormatter::new();
    let serializer = Serializer { writer, formatter };
    let mut map_key_serializer = MapKeySerializer { ser: &mut serializer };
    map_key_serializer.serialize_f64(3.141592653589793);
}

#[test]
fn test_serialize_f64_large_negative() {
    let mut writer = Vec::new();
    let formatter = CompactFormatter::new();
    let serializer = Serializer { writer, formatter };
    let mut map_key_serializer = MapKeySerializer { ser: &mut serializer };
    map_key_serializer.serialize_f64(-1.7976931348623157e+308);
}

