// Answer 0

#[test]
fn test_new_invalid_unprintable_byte() {
    // Given an invalid alphabet string containing a non-printable byte.
    let invalid_alphabet = "\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F\x10\
                            \x11\x12\x13\x14\x15\x16\x17\x18\x19\x1A\x1B\x1C\x1D\x1E\x1F\
                            ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    // Act
    let result = base64::new(invalid_alphabet);

    // Assert
    assert!(result.is_err());
    if let Err(e) = result {
        match e {
            base64::ParseAlphabetError::UnprintableByte(byte) => {
                assert_eq!(byte, 1); // Ensure that the first unprintable byte is correctly identified
            },
            _ => panic!("Expected UnprintableByte error, but got a different error."),
        }
    }
}

#[test]
fn test_new_invalid_upper_bound() {
    // Given an invalid alphabet string where one byte is just above the allowed range.
    let invalid_alphabet = " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz{|}~";

    // Act
    let result = base64::new(invalid_alphabet);

    // Assert
    assert!(result.is_err());
    if let Err(e) = result {
        match e {
            base64::ParseAlphabetError::UnprintableByte(byte) => {
                assert_eq!(byte, 127); // Ensure that byte 127 is correctly identified as unprintable
            },
            _ => panic!("Expected UnprintableByte error, but got a different error."),
        }
    }
}

