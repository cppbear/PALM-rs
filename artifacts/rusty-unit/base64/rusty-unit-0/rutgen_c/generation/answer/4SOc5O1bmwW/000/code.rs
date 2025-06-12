// Answer 0

#[test]
fn test_as_str_standard_alphabet() {
    let alphabet = Alphabet::from_str_unchecked("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/");
    assert_eq!(alphabet.as_str(), "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/");
}

#[test]
fn test_as_str_url_safe_alphabet() {
    let alphabet = Alphabet::from_str_unchecked("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_");
    assert_eq!(alphabet.as_str(), "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_");
}

#[test]
fn test_as_str_crypt_alphabet() {
    let alphabet = Alphabet::from_str_unchecked("./0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz");
    assert_eq!(alphabet.as_str(), "./0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz");
}

#[test]
fn test_as_str_bcrypt_alphabet() {
    let alphabet = Alphabet::from_str_unchecked("./ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789");
    assert_eq!(alphabet.as_str(), "./ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789");
}

#[test]
fn test_as_str_imap_mut7_alphabet() {
    let alphabet = Alphabet::from_str_unchecked("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+,");
    assert_eq!(alphabet.as_str(), "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+,");

}

#[test]
fn test_as_str_bin_hex_alphabet() {
    let alphabet = Alphabet::from_str_unchecked("!\"#$%&'()*+,-012345689@ABCDEFGHIJKLMNPQRSTUVXYZ[`abcdefhijklmpqr");
    assert_eq!(alphabet.as_str(), "!\"#$%&'()*+,-012345689@ABCDEFGHIJKLMNPQRSTUVXYZ[`abcdefhijklmpqr");
}

