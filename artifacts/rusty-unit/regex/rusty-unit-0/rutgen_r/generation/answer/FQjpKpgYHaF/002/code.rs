// Answer 0

#[derive(Debug)]
struct MyChar(u32);

impl std::fmt::Display for MyChar {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match char::from_u32(self.0) {
            None => write!(f, "Empty"),
            Some(c) => write!(f, "{:?}", c),
        }
    }
}

#[test]
fn test_fmt_none_case() {
    let character = MyChar(0x110000); // Value outside valid char range
    let result = format!("{}", character);
    assert_eq!(result, "Empty");
}

#[test]
fn test_fmt_minimum_case() {
    let character = MyChar(0xD800); // Surrogate range start, which is also invalid
    let result = format!("{}", character);
    assert_eq!(result, "Empty");
}

#[test]
fn test_fmt_negative_case() {
    let character = MyChar(0xFFFFFFFF); // Outside of valid u32 range for char
    let result = format!("{}", character);
    assert_eq!(result, "Empty");
}

