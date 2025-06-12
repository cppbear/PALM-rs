pub const fn new(alphabet: &str) -> Result<Self, ParseAlphabetError> {
        let bytes = alphabet.as_bytes();
        if bytes.len() != ALPHABET_SIZE {
            return Err(ParseAlphabetError::InvalidLength);
        }

        {
            let mut index = 0;
            while index < ALPHABET_SIZE {
                let byte = bytes[index];

                // must be ascii printable. 127 (DEL) is commonly considered printable
                // for some reason but clearly unsuitable for base64.
                if !(byte >= 32_u8 && byte <= 126_u8) {
                    return Err(ParseAlphabetError::UnprintableByte(byte));
                }
                // = is assumed to be padding, so cannot be used as a symbol
                if byte == PAD_BYTE {
                    return Err(ParseAlphabetError::ReservedByte(byte));
                }

                // Check for duplicates while staying within what const allows.
                // It's n^2, but only over 64 hot bytes, and only once, so it's likely in the single digit
                // microsecond range.

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