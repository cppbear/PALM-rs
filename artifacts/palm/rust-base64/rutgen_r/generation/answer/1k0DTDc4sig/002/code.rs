// Answer 0

#[derive(Debug)]
struct Alphabet {
    symbols: [u8; 64],
}

const ALPHABET_SIZE: usize = 64;

impl Alphabet {
    const fn from_str_unchecked(alphabet: &str) -> Self {
        let mut symbols = [0_u8; ALPHABET_SIZE];
        let source_bytes = alphabet.as_bytes();

        let mut index = 0;
        while index < ALPHABET_SIZE {
            symbols[index] = source_bytes[index];
            index += 1;
        }

        Self { symbols }
    }
}

#[test]
fn test_from_str_unchecked_valid_input() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let expected = Alphabet {
        symbols: [
            b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H',
            b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P',
            b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X',
            b'Y', b'Z', b'a', b'b', b'c', b'd', b'e', b'f',
            b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n',
            b'o', b'p', b'q', b'r', b's', b't', b'u', b'v',
            b'w', b'x', b'y', b'z', b'0', b'1', b'2', b'3',
            b'4', b'5', b'6', b'7', b'8', b'9', b'+', b'/',
        ],
    };
    let result = Alphabet::from_str_unchecked(alphabet);
    assert_eq!(result.symbols, expected.symbols);
}

#[should_panic]
#[test]
fn test_from_str_unchecked_exceeding_length() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/extra_char";
    Alphabet::from_str_unchecked(alphabet);
}

#[should_panic]
#[test]
fn test_from_str_unchecked_empty_string() {
    let alphabet = "";
    Alphabet::from_str_unchecked(alphabet);
}

