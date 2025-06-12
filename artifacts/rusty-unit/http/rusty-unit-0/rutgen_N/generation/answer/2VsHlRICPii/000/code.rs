// Answer 0

#[derive(Debug)]
struct ByteStr {
    bytes: Bytes,
}

struct Bytes {
    data: &'static [u8],
}

impl Bytes {
    const fn from_static(data: &'static [u8]) -> Self {
        Bytes { data }
    }
}

#[test]
fn test_from_static() {
    let input: &'static str = "Hello, World!";
    
    let result = from_static(input);
    assert_eq!(result.bytes.data, input.as_bytes());
}

#[test]
fn test_from_static_empty() {
    let input: &'static str = "";
    
    let result = from_static(input);
    assert_eq!(result.bytes.data, input.as_bytes());
}

#[test]
#[should_panic]
fn test_from_static_invalid_utf8() {
    let input: &'static [u8] = &[0xff, 0xff]; // Invalid UTF-8
    let input_str = unsafe { std::str::from_utf8_unchecked(input) };
    
    let _ = from_static(input_str);
}

