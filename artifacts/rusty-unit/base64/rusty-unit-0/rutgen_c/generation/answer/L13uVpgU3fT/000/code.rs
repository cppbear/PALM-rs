// Answer 0

#[test]
fn test_decode_table_with_standard_alphabet() {
    let alphabet = Alphabet {
        symbols: *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/",
    };
    let decode_table = decode_table(&alphabet);
    
    assert_eq!(decode_table[b'A' as usize], 0);
    assert_eq!(decode_table[b'Z' as usize], 25);
    assert_eq!(decode_table[b'a' as usize], 26);
    assert_eq!(decode_table[b'z' as usize], 51);
    assert_eq!(decode_table[b'0' as usize], 52);
    assert_eq!(decode_table[b'9' as usize], 61);
    assert_eq!(decode_table[b'+'] as usize, 62);
    assert_eq!(decode_table[b'/'] as usize, 63);
}

#[test]
fn test_decode_table_with_invalid_bytes() {
    let alphabet = Alphabet {
        symbols: *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/",
    };
    let decode_table = decode_table(&alphabet);
    
    assert_eq!(decode_table[b'!' as usize], INVALID_VALUE);
    assert_eq!(decode_table[b'@' as usize], INVALID_VALUE);
    assert_eq!(decode_table[b'[' as usize], INVALID_VALUE);
    assert_eq!(decode_table[b'`' as usize], INVALID_VALUE);
    assert_eq!(decode_table[b'{' as usize], INVALID_VALUE);
}

#[test]
fn test_decode_table_with_all_bytes() {
    let alphabet = Alphabet {
        symbols: *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/",
    };
    let decode_table = decode_table(&alphabet);
    
    for byte in 0..256 {
        if (b'A'..=b'Z').contains(&(byte as u8)) || 
           (b'a'..=b'z').contains(&(byte as u8)) || 
           (b'0'..=b'9').contains(&(byte as u8)) || 
           byte == b'+' as usize || byte == b'/' as usize {
            continue; // Skip valid alphabet bytes
        }
        assert_eq!(decode_table[byte], INVALID_VALUE);
    }
}

