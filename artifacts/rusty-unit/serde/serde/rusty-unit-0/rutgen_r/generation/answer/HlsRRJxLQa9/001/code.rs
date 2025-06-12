// Answer 0

#[derive(Debug)]
struct Content {
    value: i32,
}

impl Content {
    fn I32(v: i32) -> Self {
        Content { value: v }
    }
}

#[derive(Debug)]
struct SerdeError;

fn serialize_i32(v: i32) -> Result<Content, SerdeError> {
    Ok(Content::I32(v))
}

#[test]
fn test_serialize_i32_positive() {
    let result = serialize_i32(42);
    assert_eq!(result, Ok(Content::I32(42)));
}

#[test]
fn test_serialize_i32_negative() {
    let result = serialize_i32(-42);
    assert_eq!(result, Ok(Content::I32(-42)));
}

#[test]
fn test_serialize_i32_zero() {
    let result = serialize_i32(0);
    assert_eq!(result, Ok(Content::I32(0)));
}

#[test]
fn test_serialize_i32_max() {
    let result = serialize_i32(i32::MAX);
    assert_eq!(result, Ok(Content::I32(i32::MAX)));
}

#[test]
fn test_serialize_i32_min() {
    let result = serialize_i32(i32::MIN);
    assert_eq!(result, Ok(Content::I32(i32::MIN)));
}

