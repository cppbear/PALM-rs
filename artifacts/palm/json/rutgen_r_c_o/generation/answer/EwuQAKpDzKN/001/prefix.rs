// Answer 0

#[test]
fn test_serialize_map_none() {
    let serializer = MapKeySerializer;
    let _ = serializer.serialize_map(None);
}

#[test]
fn test_serialize_map_zero() {
    let serializer = MapKeySerializer;
    let _ = serializer.serialize_map(Some(0));
}

#[test]
fn test_serialize_map_one() {
    let serializer = MapKeySerializer;
    let _ = serializer.serialize_map(Some(1));
}

#[test]
fn test_serialize_map_max() {
    let serializer = MapKeySerializer;
    let _ = serializer.serialize_map(Some(usize::MAX));
}

#[test]
fn test_serialize_map_max_minus_one() {
    let serializer = MapKeySerializer;
    let _ = serializer.serialize_map(Some(usize::MAX - 1));
}

