// Answer 0

#[derive(Debug, PartialEq)]
struct Byte(u8);

impl Byte {
    fn eof() -> Self {
        Byte(256)
    }
}

#[test]
fn test_eof() {
    let result = Byte::eof();
    assert_eq!(result, Byte(256));
}

#[test]
fn test_eof_byte_value() {
    let result = Byte::eof();
    assert!(result.0 > 255); // Edge case, since u8 can only go up to 255
}

