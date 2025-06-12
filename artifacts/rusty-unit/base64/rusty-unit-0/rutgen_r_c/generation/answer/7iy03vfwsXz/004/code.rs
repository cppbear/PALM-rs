// Answer 0

#[test]
fn test_invalid_length_display() {
    use crate::ParseAlphabetError;

    let error = ParseAlphabetError::InvalidLength;
    let mut output = String::new();
    let result = error.fmt(&mut output);
    
    assert!(result.is_ok());
    assert_eq!(output, "Invalid length - must be 64 bytes");
}

#[test]
fn test_duplicated_byte_display() {
    use crate::ParseAlphabetError;

    let error = ParseAlphabetError::DuplicatedByte(0xFF);
    let mut output = String::new();
    let result = error.fmt(&mut output);
    
    assert!(result.is_ok());
    assert_eq!(output, "Duplicated byte: 0xff");
}

#[test]
fn test_unprintable_byte_display() {
    use crate::ParseAlphabetError;

    let error = ParseAlphabetError::UnprintableByte(0x1F);
    let mut output = String::new();
    let result = error.fmt(&mut output);
    
    assert!(result.is_ok());
    assert_eq!(output, "Unprintable byte: 0x1f");
}

#[test]
fn test_reserved_byte_display() {
    use crate::ParseAlphabetError;

    let error = ParseAlphabetError::ReservedByte(b'=');
    let mut output = String::new();
    let result = error.fmt(&mut output);
    
    assert!(result.is_ok());
    assert_eq!(output, "Reserved byte: 0x3d");
}

