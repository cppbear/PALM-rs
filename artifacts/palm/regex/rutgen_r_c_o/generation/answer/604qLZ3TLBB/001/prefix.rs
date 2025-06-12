// Answer 0

#[test]
fn test_is_word_char_valid_cases() {
    let valid_chars = [
        Char(0), 
        Char(1), 
        Char(127), 
        Char(128), 
        Char(255), 
        Char(256), 
        Char(1023), 
        Char(2047), 
        Char(65535), 
        Char(65536), 
        Char(1114111)
    ];
    
    for &c in valid_chars.iter() {
        let _ = c.is_word_char();
    }
}

#[test]
fn test_is_word_char_invalid_case() {
    let invalid_char = Char(1114112);
    let _ = invalid_char.is_word_char();
}

#[test]
fn test_is_word_char_max_int() {
    let max_int_char = Char(2147483647);
    let _ = max_int_char.is_word_char();
}

