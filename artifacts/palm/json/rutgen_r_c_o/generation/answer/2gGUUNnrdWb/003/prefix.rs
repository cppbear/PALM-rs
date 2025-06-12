// Answer 0

#[test]
fn test_serialize_u32_zero() {
    let mut output = Vec::new();
    let formatter = CompactFormatter::default();
    let mut serializer = Serializer::new(&mut output, formatter);
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };
    map_key_serializer.serialize_u32(0);
}

#[test]
fn test_serialize_u32_max() {
    let mut output = Vec::new();
    let formatter = CompactFormatter::default();
    let mut serializer = Serializer::new(&mut output, formatter);
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };
    map_key_serializer.serialize_u32(4294967295);
}

#[test]
fn test_serialize_u32_middle() {
    let mut output = Vec::new();
    let formatter = CompactFormatter::default();
    let mut serializer = Serializer::new(&mut output, formatter);
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };
    map_key_serializer.serialize_u32(2147483648);
}

#[test]
fn test_serialize_u32_small_positive() {
    let mut output = Vec::new();
    let formatter = CompactFormatter::default();
    let mut serializer = Serializer::new(&mut output, formatter);
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };
    map_key_serializer.serialize_u32(123);
}

#[test]
fn test_serialize_u32_large_positive() {
    let mut output = Vec::new();
    let formatter = CompactFormatter::default();
    let mut serializer = Serializer::new(&mut output, formatter);
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };
    map_key_serializer.serialize_u32(987654321);
}

