// Answer 0

#[test]
fn test_serialize_f64_valid() {
    let writer = vec![];
    let formatter = CompactFormatter {};
    let mut serializer = Serializer { writer, formatter };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };
    
    map_key_serializer.serialize_f64(0.0);
    map_key_serializer.serialize_f64(1.0);
    map_key_serializer.serialize_f64(-1.0);
    map_key_serializer.serialize_f64(1.7976931348623157e+308);
    map_key_serializer.serialize_f64(-1.7976931348623157e+308);
}

#[test]
#[should_panic]
fn test_serialize_f64_infinite() {
    let writer = vec![];
    let formatter = CompactFormatter {};
    let mut serializer = Serializer { writer, formatter };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };
    
    map_key_serializer.serialize_f64(f64::INFINITY);
}

#[test]
#[should_panic]
fn test_serialize_f64_nan() {
    let writer = vec![];
    let formatter = CompactFormatter {};
    let mut serializer = Serializer { writer, formatter };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };
    
    map_key_serializer.serialize_f64(f64::NAN);
}

