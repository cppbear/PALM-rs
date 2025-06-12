// Answer 0

#[test]
fn test_invalid_last_symbol() {
    use std::fmt;

    #[derive(Debug)]
    enum Base64Error {
        InvalidByte(usize, u8),
        InvalidLength(usize),
        InvalidLastSymbol(usize, u8),
        InvalidPadding,
    }

    impl fmt::Display for Base64Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                Self::InvalidByte(index, byte) => {
                    write!(f, "Invalid symbol {}, offset {}.", byte, index)
                }
                Self::InvalidLength(len) => write!(f, "Invalid input length: {}", len),
                Self::InvalidLastSymbol(index, byte) => {
                    write!(f, "Invalid last symbol {}, offset {}.", byte, index)
                }
                Self::InvalidPadding => write!(f, "Invalid padding"),
            }
        }
    }

    let error = Base64Error::InvalidLastSymbol(5, 0xFF);
    let result = format!("{}", error);
    assert_eq!(result, "Invalid last symbol 255, offset 5.");
}

#[test]
fn test_invalid_last_symbol_edge_case() {
    use std::fmt;

    #[derive(Debug)]
    enum Base64Error {
        InvalidByte(usize, u8),
        InvalidLength(usize),
        InvalidLastSymbol(usize, u8),
        InvalidPadding,
    }

    impl fmt::Display for Base64Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                Self::InvalidByte(index, byte) => {
                    write!(f, "Invalid symbol {}, offset {}.", byte, index)
                }
                Self::InvalidLength(len) => write!(f, "Invalid input length: {}", len),
                Self::InvalidLastSymbol(index, byte) => {
                    write!(f, "Invalid last symbol {}, offset {}.", byte, index)
                }
                Self::InvalidPadding => write!(f, "Invalid padding"),
            }
        }
    }

    let error = Base64Error::InvalidLastSymbol(0, 0x00);
    let result = format!("{}", error);
    assert_eq!(result, "Invalid last symbol 0, offset 0.");
}

