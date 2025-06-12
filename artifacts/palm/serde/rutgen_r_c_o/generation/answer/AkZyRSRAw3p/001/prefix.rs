// Answer 0

#[test]
fn test_serialize_i8_negative_boundary() {
    let mut map: HashMap<String, String> = HashMap::new();
    let serializer = FlatMapSerializer(&mut map);
    serializer.serialize_i8(-128);
}

#[test]
fn test_serialize_i8_zero() {
    let mut map: HashMap<String, String> = HashMap::new();
    let serializer = FlatMapSerializer(&mut map);
    serializer.serialize_i8(0);
}

#[test]
fn test_serialize_i8_positive_boundary() {
    let mut map: HashMap<String, String> = HashMap::new();
    let serializer = FlatMapSerializer(&mut map);
    serializer.serialize_i8(127);
}

#[test]
fn test_serialize_i8_negative_value() {
    let mut map: HashMap<String, String> = HashMap::new();
    let serializer = FlatMapSerializer(&mut map);
    serializer.serialize_i8(-1);
}

#[test]
fn test_serialize_i8_positive_value() {
    let mut map: HashMap<String, String> = HashMap::new();
    let serializer = FlatMapSerializer(&mut map);
    serializer.serialize_i8(1);
}

