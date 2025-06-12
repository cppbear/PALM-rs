// Answer 0

#[test]
fn test_serialize_f32_positive() {
    let writer = Vec::new();
    let mut serializer = Serializer { writer };
    serializer.serialize_f32(1.0);
}

#[test]
fn test_serialize_f32_negative() {
    let writer = Vec::new();
    let mut serializer = Serializer { writer };
    serializer.serialize_f32(-1.0);
}

#[test]
fn test_serialize_f32_zero() {
    let writer = Vec::new();
    let mut serializer = Serializer { writer };
    serializer.serialize_f32(0.0);
}

#[test]
fn test_serialize_f32_small() {
    let writer = Vec::new();
    let mut serializer = Serializer { writer };
    serializer.serialize_f32(-3.4028235e+38);
}

#[test]
fn test_serialize_f32_large() {
    let writer = Vec::new();
    let mut serializer = Serializer { writer };
    serializer.serialize_f32(3.4028235e+38);
}

