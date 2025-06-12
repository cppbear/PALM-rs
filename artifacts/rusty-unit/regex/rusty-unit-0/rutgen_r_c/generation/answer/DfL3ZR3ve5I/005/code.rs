// Answer 0

#[test]
fn test_fmt_with_control_characters() {
    use std::fmt::Formatter;

    struct MockFormatter {
        output: String,
    }

    impl Formatter for MockFormatter {
        fn debug_struct(&self, _name: &str) -> &Self {
            self
        }

        fn field(&self, _key: &str, _value: &String) -> &Self {
            self
        }

        fn finish(self) -> fmt::Result {
            Ok(())
        }
    }

    let start = char::from_u32(0x0000).unwrap(); // Control character
    let end = char::from_u32(0x007F).unwrap(); // Control character
    let unicode_range = ClassUnicodeRange { start, end };

    let mut formatter = MockFormatter { output: String::new() };

    let result = unicode_range.fmt(&mut formatter);
    assert!(result.is_ok());
}

#[test]
fn test_fmt_with_space_character() {
    use std::fmt::Formatter;

    struct MockFormatter {
        output: String,
    }

    impl Formatter for MockFormatter {
        fn debug_struct(&self, _name: &str) -> &Self {
            self
        }

        fn field(&self, _key: &str, _value: &String) -> &Self {
            self
        }

        fn finish(self) -> fmt::Result {
            Ok(())
        }
    }

    let start = char::from_u32(0x0009).unwrap(); // Tab character (whitespace)
    let end = char::from_u32(0x000A).unwrap(); // New line character (whitespace)
    let unicode_range = ClassUnicodeRange { start, end };

    let mut formatter = MockFormatter { output: String::new() };

    let result = unicode_range.fmt(&mut formatter);
    assert!(result.is_ok());
}

