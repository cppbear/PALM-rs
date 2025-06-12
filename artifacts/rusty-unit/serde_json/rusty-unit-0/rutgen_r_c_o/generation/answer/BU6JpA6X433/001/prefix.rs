// Answer 0

#[test]
fn test_serialize_some_bool_true() {
    let serializer = Serializer;
    let value = true;
    serializer.serialize_some(&value);
}

#[test]
fn test_serialize_some_bool_false() {
    let serializer = Serializer;
    let value = false;
    serializer.serialize_some(&value);
}

#[test]
fn test_serialize_some_i8() {
    let serializer = Serializer;
    for value in -128i8..=127 {
        serializer.serialize_some(&value);
    }
}

#[test]
fn test_serialize_some_i16() {
    let serializer = Serializer;
    for value in -32768i16..=32767 {
        serializer.serialize_some(&value);
    }
}

#[test]
fn test_serialize_some_i32() {
    let serializer = Serializer;
    for value in -2147483648i32..=2147483647 {
        serializer.serialize_some(&value);
    }
}

#[test]
fn test_serialize_some_i64() {
    let serializer = Serializer;
    for value in -9223372036854775808i64..=9223372036854775807 {
        serializer.serialize_some(&value);
    }
}

#[test]
fn test_serialize_some_i128() {
    let serializer = Serializer;
    for value in [
        -170141183460469231731687303715884105728i128,
        0,
        170141183460469231731687303715884105727i128,
    ].iter() {
        serializer.serialize_some(value);
    }
}

#[test]
fn test_serialize_some_u8() {
    let serializer = Serializer;
    for value in 0u8..=255 {
        serializer.serialize_some(&value);
    }
}

#[test]
fn test_serialize_some_u16() {
    let serializer = Serializer;
    for value in 0u16..=65535 {
        serializer.serialize_some(&value);
    }
}

#[test]
fn test_serialize_some_u32() {
    let serializer = Serializer;
    for value in 0u32..=4294967295 {
        serializer.serialize_some(&value);
    }
}

#[test]
fn test_serialize_some_u64() {
    let serializer = Serializer;
    for value in 0u64..=18446744073709551615 {
        serializer.serialize_some(&value);
    }
}

#[test]
fn test_serialize_some_u128() {
    let serializer = Serializer;
    for value in [
        0u128,
        340282366920938463463374607431768211455u128,
    ].iter() {
        serializer.serialize_some(value);
    }
}

#[test]
fn test_serialize_some_f32() {
    let serializer = Serializer;
    let values = vec![-3.4028235e38, 0.0, 3.4028235e38]; // Min and max f32
    for value in values {
        serializer.serialize_some(&value);
    }
}

#[test]
fn test_serialize_some_f64() {
    let serializer = Serializer;
    let values = vec![-1.7976931348623157e308, 0.0, 1.7976931348623157e308]; // Min and max f64
    for value in values {
        serializer.serialize_some(&value);
    }
}

#[test]
fn test_serialize_some_char() {
    let serializer = Serializer;
    let values = ['a', 'Z', '\u{1F600}']; // various valid unicode characters
    for value in values {
        serializer.serialize_some(&value);
    }
}

#[test]
fn test_serialize_some_str() {
    let serializer = Serializer;
    let values = vec![
        "", 
        "short string", 
        "a".repeat(65536) // 2^16 characters
    ];
    for value in values {
        serializer.serialize_some(value);
    }
}

#[test]
fn test_serialize_some_bytes() {
    let serializer = Serializer;
    let values = vec![
        &[],
        &[0u8; 65536], // 2^16 bytes
    ];
    for value in values {
        serializer.serialize_some(value);
    }
}

#[test]
fn test_serialize_some_unit() {
    let serializer = Serializer;
    let value = ();
    serializer.serialize_some(&value);
}

#[test]
fn test_serialize_some_option_none() {
    let serializer = Serializer;
    let value: Option<i32> = None;
    serializer.serialize_some(&value);
}

#[test]
fn test_serialize_some_option_some() {
    let serializer = Serializer;
    let value: Option<i32> = Some(42);
    serializer.serialize_some(&value);
}

#[test]
fn test_serialize_some_vec_empty() {
    let serializer = Serializer;
    let value: Vec<i32> = Vec::new();
    serializer.serialize_some(&value);
}

#[test]
fn test_serialize_some_vec_filled() {
    let serializer = Serializer;
    let value: Vec<i32> = (0..100).collect();
    serializer.serialize_some(&value);
}

#[test]
fn test_serialize_some_map_empty() {
    let serializer = Serializer;
    let value: Map<String, Value> = Map::new();
    serializer.serialize_some(&value);
}

#[test]
fn test_serialize_some_map_filled() {
    let serializer = Serializer;
    let mut value = Map::new();
    for i in 0..100 {
        value.insert(format!("key{}", i), Value::Number(i.into()));
    }
    serializer.serialize_some(&value);
}

