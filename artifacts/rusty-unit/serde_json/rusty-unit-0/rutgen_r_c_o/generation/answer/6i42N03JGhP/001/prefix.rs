// Answer 0

#[test]
fn test_serialize_u128_panic_case_1() {
    let writer = Vec::new();
    let formatter = CompactFormatter;
    let mut serializer = Serializer { writer, formatter };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };
    
    let value = 0; // minimal value
    map_key_serializer.serialize_u128(value);
}

#[test]
fn test_serialize_u128_panic_case_2() {
    let writer = Vec::new();
    let formatter = CompactFormatter;
    let mut serializer = Serializer { writer, formatter };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };
    
    let value = 18446744073709551615; // maximal value
    map_key_serializer.serialize_u128(value);
}

#[test]
fn test_serialize_u128_panic_case_3() {
    let writer = Vec::new();
    let formatter = CompactFormatter;
    let mut serializer = Serializer { writer, formatter };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };
    
    let value = 12345678901234567890; // arbitrary value within range
    map_key_serializer.serialize_u128(value);
}

