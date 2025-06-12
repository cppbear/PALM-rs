// Answer 0

#[test]
fn test_alphabet_with_unprintable_byte_above_limit() {
    let result = Alphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/" 
                                + &String::from_utf8(vec![127]).unwrap());
}

#[test]
fn test_alphabet_with_unprintable_byte_high_value() {
    let result = Alphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/" 
                                + &String::from_utf8(vec![255]).unwrap());
}

#[test]
fn test_alphabet_with_multiple_unprintable_bytes() {
    let result = Alphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
                                + &String::from_utf8(vec![128, 129, 130]).unwrap());
}

#[test]
fn test_alphabet_with_all_high_unprintable_bytes() {
    let result = Alphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/" 
                                + &String::from_utf8((127..=255).collect::<Vec<u8>>()).unwrap());
}

