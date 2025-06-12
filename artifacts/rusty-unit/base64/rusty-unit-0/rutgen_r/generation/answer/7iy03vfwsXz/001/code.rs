// Answer 0

#[test]
fn test_fmt_invalid_length() {
    enum Alphabet {
        InvalidLength,
        DuplicatedByte(u8),
        UnprintableByte(u8),
        ReservedByte(u8),
    }
    
    let invalid_length = Alphabet::InvalidLength;
    let mut output = String::new();
    let result = std::fmt::write(&mut output, |f| invalid_length.fmt(f));
    assert!(result.is_ok());
    assert_eq!(output, "Invalid length - must be 64 bytes");
}

#[test]
fn test_fmt_duplicated_byte() {
    enum Alphabet {
        InvalidLength,
        DuplicatedByte(u8),
        UnprintableByte(u8),
        ReservedByte(u8),
    }

    let duplicated_byte = Alphabet::DuplicatedByte(0xFF);
    let mut output = String::new();
    let result = std::fmt::write(&mut output, |f| duplicated_byte.fmt(f));
    assert!(result.is_ok());
    assert_eq!(output, "Duplicated byte: 0xff");
}

#[test]
fn test_fmt_unprintable_byte() {
    enum Alphabet {
        InvalidLength,
        DuplicatedByte(u8),
        UnprintableByte(u8),
        ReservedByte(u8),
    }
    
    let unprintable_byte = Alphabet::UnprintableByte(0x7F);
    let mut output = String::new();
    let result = std::fmt::write(&mut output, |f| unprintable_byte.fmt(f));
    assert!(result.is_ok());
    assert_eq!(output, "Unprintable byte: 0x7f");
}

#[test]
fn test_fmt_reserved_byte() {
    enum Alphabet {
        InvalidLength,
        DuplicatedByte(u8),
        UnprintableByte(u8),
        ReservedByte(u8),
    }

    let reserved_byte = Alphabet::ReservedByte(0x1A);
    let mut output = String::new();
    let result = std::fmt::write(&mut output, |f| reserved_byte.fmt(f));
    assert!(result.is_ok());
    assert_eq!(output, "Reserved byte: 0x1a");
}

