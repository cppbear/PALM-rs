// Answer 0

#[test]
fn test_serialize_u32() {
    let value: u32 = 0;
    let result = serialize_u32(value);
    assert_eq!(result, Ok("0".to_string()));
}

#[test]
fn test_serialize_u32_boundaries() {
    let value: u32 = u32::MAX;
    let result = serialize_u32(value);
    assert_eq!(result, Ok(itoa::Buffer::new().format(value).to_owned()));
}

#[test]
fn test_serialize_u32_smallest_non_zero() {
    let value: u32 = 1;
    let result = serialize_u32(value);
    assert_eq!(result, Ok("1".to_string()));
}

#[test]
fn test_serialize_u32_large_value() {
    let value: u32 = 123456;
    let result = serialize_u32(value);
    assert_eq!(result, Ok("123456".to_string()));
}

