// Answer 0

#[test]
fn test_serialize_u32_valid() {
    fn serialize_u32(value: u32) -> Result<String, std::io::Error> {
        Ok(itoa::Buffer::new().format(value).to_owned())
    }

    let value = 42;
    let result = serialize_u32(value);
    assert_eq!(result, Ok("42".to_string()));
}

#[test]
fn test_serialize_u32_zero() {
    fn serialize_u32(value: u32) -> Result<String, std::io::Error> {
        Ok(itoa::Buffer::new().format(value).to_owned())
    }

    let value = 0;
    let result = serialize_u32(value);
    assert_eq!(result, Ok("0".to_string()));
}

#[test]
fn test_serialize_u32_max() {
    fn serialize_u32(value: u32) -> Result<String, std::io::Error> {
        Ok(itoa::Buffer::new().format(value).to_owned())
    }

    let value = u32::MAX;
    let result = serialize_u32(value);
    assert_eq!(result, Ok(u32::MAX.to_string()));
}

