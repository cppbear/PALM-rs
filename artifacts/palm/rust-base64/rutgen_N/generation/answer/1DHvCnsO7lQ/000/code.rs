// Answer 0

#[derive(Debug)]
struct Alphabet {
    characters: Vec<char>,
}

struct GeneralPurposeConfig {
    // Configuration fields here
}

struct GeneralPurpose {
    encode_table: Vec<u8>,
    decode_table: Vec<u8>,
    config: GeneralPurposeConfig,
}

impl GeneralPurpose {
    pub const fn new(alphabet: &Alphabet, config: GeneralPurposeConfig) -> Self {
        Self {
            encode_table: Self::encode_table(alphabet),
            decode_table: Self::decode_table(alphabet),
            config,
        }
    }

    const fn encode_table(alphabet: &Alphabet) -> Vec<u8> {
        // This should return a vector based on the alphabet characters
        alphabet.characters.iter().map(|c| *c as u8).collect()
    }

    const fn decode_table(alphabet: &Alphabet) -> Vec<u8> {
        // This should return a vector based on the alphabet characters
        alphabet.characters.iter().map(|c| *c as u8).collect()
    }
}

#[test]
fn test_general_purpose_new() {
    let alphabet = Alphabet {
        characters: vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'],
    };
    
    let config = GeneralPurposeConfig {
        // Initialize configuration fields as necessary
    };

    let engine = GeneralPurpose::new(&alphabet, config);
    
    assert_eq!(engine.encode_table.len(), 26);
    assert_eq!(engine.decode_table.len(), 26);
}

