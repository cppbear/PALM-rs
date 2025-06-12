// Answer 0

#[test]
fn test_duplicate_byte_in_alphabet() {
    struct Alphabet;

    impl Alphabet {
        pub const fn new(alphabet: &str) -> Result<Self, ParseAlphabetError> {
            // Simulating the original function definition
            const ALPHABET_SIZE: usize = 64;
            const PAD_BYTE: u8 = b'='; // Assuming '=' is reserved
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

            Ok(Alphabet)
        }
    }

    enum ParseAlphabetError {
        InvalidLength,
        UnprintableByte(u8),
        ReservedByte(u8),
        DuplicatedByte(u8),
    }

    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    assert_eq!(Alphabet::new(alphabet), Ok(Alphabet));

    let duplicate_alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+AD"; // 'A' is duplicated
    assert_eq!(Alphabet::new(duplicate_alphabet), Err(ParseAlphabetError::DuplicatedByte(b'A')));
}

