// Answer 0

#[test]
fn test_collect_str_non_empty_string() {
    let serializer = MapKeySerializer;
    let result = serializer.collect_str("hello world");
}

#[test]
fn test_collect_str_single_character() {
    let serializer = MapKeySerializer;
    let result = serializer.collect_str('a');
}

#[test]
fn test_collect_str_boolean_true() {
    let serializer = MapKeySerializer;
    let result = serializer.collect_str(&true);
}

#[test]
fn test_collect_str_boolean_false() {
    let serializer = MapKeySerializer;
    let result = serializer.collect_str(&false);
}

#[test]
fn test_collect_str_i8() {
    let serializer = MapKeySerializer;
    let result = serializer.collect_str(&123_i8);
}

#[test]
fn test_collect_str_i16() {
    let serializer = MapKeySerializer;
    let result = serializer.collect_str(&-123_i16);
}

#[test]
fn test_collect_str_i32() {
    let serializer = MapKeySerializer;
    let result = serializer.collect_str(&2147483647_i32);
}

#[test]
fn test_collect_str_i64() {
    let serializer = MapKeySerializer;
    let result = serializer.collect_str(&-9223372036854775808_i64);
}

#[test]
fn test_collect_str_u8() {
    let serializer = MapKeySerializer;
    let result = serializer.collect_str(&255_u8);
}

#[test]
fn test_collect_str_u16() {
    let serializer = MapKeySerializer;
    let result = serializer.collect_str(&65535_u16);
}

#[test]
fn test_collect_str_u32() {
    let serializer = MapKeySerializer;
    let result = serializer.collect_str(&4294967295_u32);
}

#[test]
fn test_collect_str_f32() {
    let serializer = MapKeySerializer;
    let result = serializer.collect_str(&3.14_f32);
}

#[test]
fn test_collect_str_f64() {
    let serializer = MapKeySerializer;
    let result = serializer.collect_str(&1.7976931348623157e308_f64);
}

#[test]
fn test_collect_str_long_string() {
    let serializer = MapKeySerializer;
    let long_string = "a".repeat(1000);
    let result = serializer.collect_str(&long_string);
}

