// Answer 0

#[test]
fn test_as_str_valid_standard_alphabet() {
    let alphabet = crate::STANDARD;
    let result = alphabet.as_str();
    assert_eq!(result, "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/");
}

#[test]
fn test_as_str_valid_url_safe_alphabet() {
    let alphabet = crate::URL_SAFE;
    let result = alphabet.as_str();
    assert_eq!(result, "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_");
}

#[test]
fn test_as_str_valid_crypt_alphabet() {
    let alphabet = crate::CRYPT;
    let result = alphabet.as_str();
    assert_eq!(result, "./0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz");
}

#[test]
fn test_as_str_valid_bcrypt_alphabet() {
    let alphabet = crate::BCRYPT;
    let result = alphabet.as_str();
    assert_eq!(result, "./ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789");
}

#[test]
fn test_as_str_valid_imap_mutf7_alphabet() {
    let alphabet = crate::IMAP_MUTF7;
    let result = alphabet.as_str();
    assert_eq!(result, "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+,");
}

#[test]
fn test_as_str_valid_bin_hex_alphabet() {
    let alphabet = crate::BIN_HEX;
    let result = alphabet.as_str();
    assert_eq!(result, "!\"#$%&'()*+,-012345689@ABCDEFGHIJKLMNPQRSTUVXYZ[`abcdefhijklmpqr");
}

