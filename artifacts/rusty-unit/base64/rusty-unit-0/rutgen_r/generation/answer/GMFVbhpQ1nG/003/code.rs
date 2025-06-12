// Answer 0

#[test]
fn test_new_valid_alphabet() {
    struct ParseAlphabetError;
    impl ParseAlphabetError {
        const InvalidLength: Self = Self;
        const UnprintableByte: fn(u8) -> Self = |_| Self;
        const ReservedByte: fn(u8) -> Self = |_| Self;
        const DuplicatedByte: fn(u8) -> Self = |_| Self;
    }
    
    const ALPHABET_SIZE: usize = 64;
    const PAD_BYTE: u8 = b'=';

    struct Alphabet;

    impl Alphabet {
        pub const fn from_str_unchecked(_: &str) -> Self {
            Alphabet
        }
        
        pub const fn new(alphabet: &str) -> Result<Self, ParseAlphabetError> {
            let bytes = alphabet.as_bytes();

            if bytes.len() != ALPHABET_SIZE {
                return Err(ParseAlphabetError::InvalidLength);
            }

            {
                let mut index = 0;
                while index < ALPHABET_SIZE {
                    let byte = bytes[index];

                    if !(byte >= 32_u8 && byte <= 126_u8) {
                        return Err(ParseAlphabetError::UnprintableByte(byte));
                    }
                    if byte == PAD_BYTE {
                        return Err(ParseAlphabetError::ReservedByte(byte));
                    }

                    let mut probe_index = 0;
                    while probe_index < ALPHABET_SIZE {
                        if probe_index == index {
                            probe_index += 1;
                            continue;
                        }

                        let probe_byte = bytes[probe_index];

                        if byte == probe_byte {
                            return Err(ParseAlphabetError::DuplicatedByte(byte));
                        }

                        probe_index += 1;
                    }

                    index += 1;
                }
            }

            Ok(Self::from_str_unchecked(alphabet))
        }
    }

    let result = Alphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/");
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_new_invalid_length() {
    struct ParseAlphabetError;
    impl ParseAlphabetError {
        const InvalidLength: Self = Self;
        const UnprintableByte: fn(u8) -> Self = |_| Self;
        const ReservedByte: fn(u8) -> Self = |_| Self;
        const DuplicatedByte: fn(u8) -> Self = |_| Self;
    }
    
    const ALPHABET_SIZE: usize = 64;
    const PAD_BYTE: u8 = b'=';

    struct Alphabet;

    impl Alphabet {
        pub const fn from_str_unchecked(_: &str) -> Self {
            Alphabet
        }
        
        pub const fn new(alphabet: &str) -> Result<Self, ParseAlphabetError> {
            let bytes = alphabet.as_bytes();

            if bytes.len() != ALPHABET_SIZE {
                return Err(ParseAlphabetError::InvalidLength);
            }

            {
                let mut index = 0;
                while index < ALPHABET_SIZE {
                    let byte = bytes[index];

                    if !(byte >= 32_u8 && byte <= 126_u8) {
                        return Err(ParseAlphabetError::UnprintableByte(byte));
                    }
                    if byte == PAD_BYTE {
                        return Err(ParseAlphabetError::ReservedByte(byte));
                    }

                    let mut probe_index = 0;
                    while probe_index < ALPHABET_SIZE {
                        if probe_index == index {
                            probe_index += 1;
                            continue;
                        }

                        let probe_byte = bytes[probe_index];

                        if byte == probe_byte {
                            return Err(ParseAlphabetError::DuplicatedByte(byte));
                        }

                        probe_index += 1;
                    }

                    index += 1;
                }
            }

            Ok(Self::from_str_unchecked(alphabet))
        }
    }

    let result = Alphabet::new("short");
    assert!(result.is_err());
}

#[test]
#[should_panic]
fn test_new_unprintable_byte() {
    struct ParseAlphabetError;
    impl ParseAlphabetError {
        const InvalidLength: Self = Self;
        const UnprintableByte: fn(u8) -> Self = |_| Self;
        const ReservedByte: fn(u8) -> Self = |_| Self;
        const DuplicatedByte: fn(u8) -> Self = |_| Self;
    }
    
    const ALPHABET_SIZE: usize = 64;
    const PAD_BYTE: u8 = b'=';

    struct Alphabet;

    impl Alphabet {
        pub const fn from_str_unchecked(_: &str) -> Self {
            Alphabet
        }
        
        pub const fn new(alphabet: &str) -> Result<Self, ParseAlphabetError> {
            let bytes = alphabet.as_bytes();

            if bytes.len() != ALPHABET_SIZE {
                return Err(ParseAlphabetError::InvalidLength);
            }

            {
                let mut index = 0;
                while index < ALPHABET_SIZE {
                    let byte = bytes[index];

                    if !(byte >= 32_u8 && byte <= 126_u8) {
                        return Err(ParseAlphabetError::UnprintableByte(byte));
                    }
                    if byte == PAD_BYTE {
                        return Err(ParseAlphabetError::ReservedByte(byte));
                    }

                    let mut probe_index = 0;
                    while probe_index < ALPHABET_SIZE {
                        if probe_index == index {
                            probe_index += 1;
                            continue;
                        }

                        let probe_byte = bytes[probe_index];

                        if byte == probe_byte {
                            return Err(ParseAlphabetError::DuplicatedByte(byte));
                        }

                        probe_index += 1;
                    }

                    index += 1;
                }
            }

            Ok(Self::from_str_unchecked(alphabet))
        }
    }

    let result = Alphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/\x01");
    assert!(result.is_err());
}

#[test]
#[should_panic]
fn test_new_reserved_byte() {
    struct ParseAlphabetError;
    impl ParseAlphabetError {
        const InvalidLength: Self = Self;
        const UnprintableByte: fn(u8) -> Self = |_| Self;
        const ReservedByte: fn(u8) -> Self = |_| Self;
        const DuplicatedByte: fn(u8) -> Self = |_| Self;
    }
    
    const ALPHABET_SIZE: usize = 64;
    const PAD_BYTE: u8 = b'=';

    struct Alphabet;

    impl Alphabet {
        pub const fn from_str_unchecked(_: &str) -> Self {
            Alphabet
        }
        
        pub const fn new(alphabet: &str) -> Result<Self, ParseAlphabetError> {
            let bytes = alphabet.as_bytes();

            if bytes.len() != ALPHABET_SIZE {
                return Err(ParseAlphabetError::InvalidLength);
            }

            {
                let mut index = 0;
                while index < ALPHABET_SIZE {
                    let byte = bytes[index];

                    if !(byte >= 32_u8 && byte <= 126_u8) {
                        return Err(ParseAlphabetError::UnprintableByte(byte));
                    }
                    if byte == PAD_BYTE {
                        return Err(ParseAlphabetError::ReservedByte(byte));
                    }

                    let mut probe_index = 0;
                    while probe_index < ALPHABET_SIZE {
                        if probe_index == index {
                            probe_index += 1;
                            continue;
                        }

                        let probe_byte = bytes[probe_index];

                        if byte == probe_byte {
                            return Err(ParseAlphabetError::DuplicatedByte(byte));
                        }

                        probe_index += 1;
                    }

                    index += 1;
                }
            }

            Ok(Self::from_str_unchecked(alphabet))
        }
    }

    let result = Alphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=");
    assert!(result.is_err());
}

#[test]
#[should_panic]
fn test_new_duplicated_byte() {
    struct ParseAlphabetError;
    impl ParseAlphabetError {
        const InvalidLength: Self = Self;
        const UnprintableByte: fn(u8) -> Self = |_| Self;
        const ReservedByte: fn(u8) -> Self = |_| Self;
        const DuplicatedByte: fn(u8) -> Self = |_| Self;
    }
    
    const ALPHABET_SIZE: usize = 64;
    const PAD_BYTE: u8 = b'=';

    struct Alphabet;

    impl Alphabet {
        pub const fn from_str_unchecked(_: &str) -> Self {
            Alphabet
        }
        
        pub const fn new(alphabet: &str) -> Result<Self, ParseAlphabetError> {
            let bytes = alphabet.as_bytes();

            if bytes.len() != ALPHABET_SIZE {
                return Err(ParseAlphabetError::InvalidLength);
            }

            {
                let mut index = 0;
                while index < ALPHABET_SIZE {
                    let byte = bytes[index];

                    if !(byte >= 32_u8 && byte <= 126_u8) {
                        return Err(ParseAlphabetError::UnprintableByte(byte));
                    }
                    if byte == PAD_BYTE {
                        return Err(ParseAlphabetError::ReservedByte(byte));
                    }

                    let mut probe_index = 0;
                    while probe_index < ALPHABET_SIZE {
                        if probe_index == index {
                            probe_index += 1;
                            continue;
                        }

                        let probe_byte = bytes[probe_index];

                        if byte == probe_byte {
                            return Err(ParseAlphabetError::DuplicatedByte(byte));
                        }

                        probe_index += 1;
                    }

                    index += 1;
                }
            }

            Ok(Self::from_str_unchecked(alphabet))
        }
    }

    let result = Alphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ01234567");
    assert!(result.is_err());
}

