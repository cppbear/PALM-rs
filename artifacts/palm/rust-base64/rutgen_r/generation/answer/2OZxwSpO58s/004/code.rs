// Answer 0

#[derive(Debug)]
enum Error {
    InvalidByte(usize, u8),
    InvalidLength(usize),
    InvalidLastSymbol(usize, u8),
    InvalidPadding,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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

#[test]
fn test_invalid_byte() {
    let index = 5;
    let byte = b'A'; // ASCII representation of 'A'
    let error = Error::InvalidByte(index, byte);
    let mut output = String::new();
    
    let result = write!(&mut output, "{}", error);
    
    assert!(result.is_ok());
    assert_eq!(output, format!("Invalid symbol {}, offset {}.", byte, index));
}

#[test]
fn test_invalid_length() {
    let length = 10;
    let error = Error::InvalidLength(length);
    let mut output = String::new();
    
    let result = write!(&mut output, "{}", error);
    
    assert!(result.is_ok());
    assert_eq!(output, format!("Invalid input length: {}", length));
}

#[test]
fn test_invalid_last_symbol() {
    let index = 7;
    let byte = b'Z'; // ASCII representation of 'Z'
    let error = Error::InvalidLastSymbol(index, byte);
    let mut output = String::new();
    
    let result = write!(&mut output, "{}", error);
    
    assert!(result.is_ok());
    assert_eq!(output, format!("Invalid last symbol {}, offset {}.", byte, index));
}

#[test]
fn test_invalid_padding() {
    let error = Error::InvalidPadding;
    let mut output = String::new();
    
    let result = write!(&mut output, "{}", error);
    
    assert!(result.is_ok());
    assert_eq!(output, "Invalid padding");
}

