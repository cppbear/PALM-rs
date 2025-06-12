// Answer 0

#[test]
fn test_valid_alphabet_standard() {
    let input = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    Alphabet::try_from(input);
}

#[test]
fn test_valid_alphabet_url_safe() {
    let input = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
    Alphabet::try_from(input);
}

#[test]
fn test_valid_alphabet_crypt() {
    let input = "./0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    Alphabet::try_from(input);
}

#[test]
fn test_valid_alphabet_bcrypt() {
    let input = "./ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    Alphabet::try_from(input);
}

#[test]
fn test_valid_alphabet_imap_mut7() {
    let input = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+,";
    Alphabet::try_from(input);
}

#[test]
fn test_valid_alphabet_bin_hex() {
    let input = "!\"#$%&'()*+,-012345689@ABCDEFGHIJKLMNPQRSTUVXYZ[`abcdefhijklmpqr";
    Alphabet::try_from(input);
}

#[test]
#[should_panic]
fn test_invalid_length() {
    let input = "ShortLength";
    Alphabet::try_from(input);
}

#[test]
#[should_panic]
fn test_invalid_character_unprintable() {
    let input = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/\x01";
    Alphabet::try_from(input);
}

#[test]
#[should_panic]
fn test_invalid_character_reserved() {
    let input = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    // Assume PAD_BYTE is defined as '='
    let input_with_reserved = input.replace('/', "=");
    Alphabet::try_from(&input_with_reserved);
}

#[test]
#[should_panic]
fn test_invalid_character_duplicated() {
    let input = "ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghij";
    Alphabet::try_from(input);
}

