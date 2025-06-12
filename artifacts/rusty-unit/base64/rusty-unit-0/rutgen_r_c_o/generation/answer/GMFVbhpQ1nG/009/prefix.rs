// Answer 0

#[test]
fn test_valid_alphabet_standard() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let result = Alphabet::new(alphabet);
}

#[test]
fn test_valid_alphabet_url_safe() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
    let result = Alphabet::new(alphabet);
}

#[test]
fn test_valid_alphabet_crypt() {
    let alphabet = "./0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let result = Alphabet::new(alphabet);
}

#[test]
fn test_valid_alphabet_bcrypt() {
    let alphabet = "./ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let result = Alphabet::new(alphabet);
}

#[test]
fn test_valid_alphabet_imap_mutf7() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+,";
    let result = Alphabet::new(alphabet);
}

#[test]
fn test_valid_alphabet_bin_hex() {
    let alphabet = "!\"#$%&'()*+,-012345689@ABCDEFGHIJKLMNPQRSTUVXYZ[`abcdefhijklmpqr";
    let result = Alphabet::new(alphabet);
}

#[test]
fn test_valid_alphabet_special_chars() {
    let alphabet = "1234567890abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ+/";
    let result = Alphabet::new(alphabet);
}

