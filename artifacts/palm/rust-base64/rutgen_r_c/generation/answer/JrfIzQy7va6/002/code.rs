// Answer 0

#[test]
fn test_encode_table_with_full_alphabet() {
    struct TestAlphabet {
        symbols: [u8; 64],
    }

    let full_alphabet = TestAlphabet {
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
    
    let encoded_table = encode_table(&full_alphabet);
    
    assert_eq!(encoded_table.len(), 64);
    assert_eq!(&encoded_table, &full_alphabet.symbols);
}

#[test]
#[should_panic]
fn test_encode_table_with_invalid_index() {
    struct InvalidAlphabet {
        symbols: [u8; 65],
    }
    
    let invalid_alphabet = InvalidAlphabet {
        symbols: [
            b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H',
            b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P',
            b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X',
            b'Y', b'Z', b'a', b'b', b'c', b'd', b'e', b'f',
            b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n',
            b'o', b'p', b'q', b'r', b's', b't', b'u', b'v',
            b'w', b'x', b'y', b'z', b'0', b'1', b'2', b'3',
            b'4', b'5', b'6', b'7', b'8', b'9', b'+', b'/',
            b'!', // This makes the size of the array 65, which is invalid
        ],
    };
    
    let _ = encode_table(&invalid_alphabet);
}

