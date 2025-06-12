// Answer 0

#[test]
fn test_serialize_f64_negative_infinity() {
    let mut writer = Vec::new();
    let formatter = /* Initialize your formatter here */;
    let serializer = Serializer { writer, formatter };
    serializer.serialize_f64(f64::NEG_INFINITY);
}

#[test]
fn test_serialize_f64_positive_infinity() {
    let mut writer = Vec::new();
    let formatter = /* Initialize your formatter here */;
    let serializer = Serializer { writer, formatter };
    serializer.serialize_f64(f64::INFINITY);
}

#[test]
fn test_serialize_f64_nan() {
    let mut writer = Vec::new();
    let formatter = /* Initialize your formatter here */;
    let serializer = Serializer { writer, formatter };
    serializer.serialize_f64(f64::NAN);
}

#[test]
fn test_serialize_f64_zero() {
    let mut writer = Vec::new();
    let formatter = /* Initialize your formatter here */;
    let serializer = Serializer { writer, formatter };
    serializer.serialize_f64(0.0);
}

#[test]
fn test_serialize_f64_negative_small() {
    let mut writer = Vec::new();
    let formatter = /* Initialize your formatter here */;
    let serializer = Serializer { writer, formatter };
    serializer.serialize_f64(-1.0e-308);
}

#[test]
fn test_serialize_f64_positive_small() {
    let mut writer = Vec::new();
    let formatter = /* Initialize your formatter here */;
    let serializer = Serializer { writer, formatter };
    serializer.serialize_f64(1.0e-308);
}

#[test]
fn test_serialize_f64_negative_large() {
    let mut writer = Vec::new();
    let formatter = /* Initialize your formatter here */;
    let serializer = Serializer { writer, formatter };
    serializer.serialize_f64(-1.0e308);
}

#[test]
fn test_serialize_f64_positive_large() {
    let mut writer = Vec::new();
    let formatter = /* Initialize your formatter here */;
    let serializer = Serializer { writer, formatter };
    serializer.serialize_f64(1.0e308);
}

