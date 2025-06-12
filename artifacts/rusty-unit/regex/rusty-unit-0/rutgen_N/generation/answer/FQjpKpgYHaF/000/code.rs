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
fn test_fmt_with_valid_char() {
    let my_char = MyChar(97); // 'a' in Unicode
    let mut output = String::new();
    let result = write!(&mut output, "{}", my_char);
    
    assert!(result.is_ok());
    assert_eq!(output, "'a'");
}

#[test]
fn test_fmt_with_empty_char() {
    let my_char = MyChar(0xFFFFFFFF); // Invalid Unicode
    let mut output = String::new();
    let result = write!(&mut output, "{}", my_char);
    
    assert!(result.is_ok());
    assert_eq!(output, "Empty");
}

