// Answer 0

#[test]
fn test_fmt_valid_char() {
    struct TestChar(u32);
    
    impl std::fmt::Display for TestChar {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match char::from_u32(self.0) {
                None => write!(f, "Empty"),
                Some(c) => write!(f, "{:?}", c),
            }
        }
    }

    let valid_char = TestChar(65); // 'A' in Unicode
    let mut output = String::new();
    let result = write!(&mut output, "{}", valid_char);
    assert!(result.is_ok());
    assert_eq!(output, "\"A\"");
}

#[test]
fn test_fmt_empty_on_invalid_char() {
    struct TestChar(u32);
    
    impl std::fmt::Display for TestChar {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match char::from_u32(self.0) {
                None => write!(f, "Empty"),
                Some(c) => write!(f, "{:?}", c),
            }
        }
    }

    let invalid_char = TestChar(0x10FFFF + 1); // Invalid Unicode code point
    let mut output = String::new();
    let result = write!(&mut output, "{}", invalid_char);
    assert!(result.is_ok());
    assert_eq!(output, "Empty");
}

