// Answer 0

#[test]
fn test_invalid_byte() {
    struct InvalidByte(u32, u8);
    impl fmt::Display for InvalidByte {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Invalid symbol {}, offset {}.", self.1, self.0)
        }
    }
    let error = InvalidByte(5, 255);
    let result = format!("{}", error);
    assert_eq!(result, "Invalid symbol 255, offset 5.");
}

#[test]
fn test_invalid_length() {
    struct InvalidLength(u32);
    impl fmt::Display for InvalidLength {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Invalid input length: {}", self.0)
        }
    }
    let error = InvalidLength(10);
    let result = format!("{}", error);
    assert_eq!(result, "Invalid input length: 10");
}

#[test]
fn test_invalid_last_symbol() {
    struct InvalidLastSymbol(u32, u8);
    impl fmt::Display for InvalidLastSymbol {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Invalid last symbol {}, offset {}.", self.1, self.0)
        }
    }
    let error = InvalidLastSymbol(7, 128);
    let result = format!("{}", error);
    assert_eq!(result, "Invalid last symbol 128, offset 7.");
}

#[test]
fn test_invalid_padding() {
    struct InvalidPadding;
    impl fmt::Display for InvalidPadding {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Invalid padding")
        }
    }
    let error = InvalidPadding;
    let result = format!("{}", error);
    assert_eq!(result, "Invalid padding");
}

