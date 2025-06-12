// Answer 0

#[test]
fn test_serialize_i128_negative_max() {
    let mut writer = Vec::new();
    let formatter = CompactFormatter::new();
    
    let mut serializer = Serializer {
        writer: &mut writer,
        formatter,
    };

    let value: i128 = -9223372036854775808;
    let _ = serializer.serialize_i128(value);
}

#[test]
fn test_serialize_i128_negative_one() {
    let mut writer = Vec::new();
    let formatter = CompactFormatter::new();
    
    let mut serializer = Serializer {
        writer: &mut writer,
        formatter,
    };

    let value: i128 = -1;
    let _ = serializer.serialize_i128(value);
}

#[test]
fn test_serialize_i128_zero() {
    let mut writer = Vec::new();
    let formatter = CompactFormatter::new();
    
    let mut serializer = Serializer {
        writer: &mut writer,
        formatter,
    };

    let value: i128 = 0;
    let _ = serializer.serialize_i128(value);
}

#[test]
fn test_serialize_i128_one() {
    let mut writer = Vec::new();
    let formatter = CompactFormatter::new();
    
    let mut serializer = Serializer {
        writer: &mut writer,
        formatter,
    };

    let value: i128 = 1;
    let _ = serializer.serialize_i128(value);
}

#[test]
fn test_serialize_i128_positive_max() {
    let mut writer = Vec::new();
    let formatter = CompactFormatter::new();
    
    let mut serializer = Serializer {
        writer: &mut writer,
        formatter,
    };

    let value: i128 = 9223372036854775807;
    let _ = serializer.serialize_i128(value);
}

