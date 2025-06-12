// Answer 0

#[derive(Debug, PartialEq)]
struct Byte(u8);

fn eof() -> Byte { Byte(256) }

#[test]
fn test_eof() {
    let result = eof();
    assert_eq!(result, Byte(256));
}

