// Answer 0

#[test]
fn test_new_reserved_byte() {
    struct Alphabet {}

    impl Alphabet {
        pub const ALPHABET_SIZE: usize = 64;
        pub const PAD_BYTE: u8 = b'=';

        pub const fn from_str_unchecked(alphabet: &str) -> Self {
            Alphabet {}
        }
        
        pub const fn new(alphabet: &str) -> Result<Self, ParseAlphabetError> {
            let bytes = alphabet.as_bytes();
            if bytes.len() != Self::ALPHABET_SIZE {
                return Err(ParseAlphabetError::InvalidLength);
            }

            {
                let mut index = 0;
                while index < Self::ALPHABET_SIZE {
                    let byte = bytes[index];

                    if !(byte >= 32_u8 && byte <= 126_u8) {
                        return Err(ParseAlphabetError::UnprintableByte(byte));
                    }
                    if byte == Self::PAD_BYTE {
                        return Err(ParseAlphabetError::ReservedByte(byte));
                    }

                    let mut probe_index = 0;
                    while probe_index < Self::ALPHABET_SIZE {
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

    enum ParseAlphabetError {
        InvalidLength,
        UnprintableByte(u8),
        ReservedByte(u8),
        DuplicatedByte(u8),
    }
    
    // Test input covering boundary conditions where `=` (PAD_BYTE) is used.
    let result = Alphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/");
    assert_eq!(result, Err(ParseAlphabetError::ReservedByte(b'=')));
}

