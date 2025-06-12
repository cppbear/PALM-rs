// Answer 0

#[test]
fn test_standard_as_str() {
    let alphabet = Alphabet::STANDARD;
    let result = alphabet.as_str();
}

#[test]
fn test_url_safe_as_str() {
    let alphabet = Alphabet::URL_SAFE;
    let result = alphabet.as_str();
}

#[test]
fn test_crypt_as_str() {
    let alphabet = Alphabet::CRYPT;
    let result = alphabet.as_str();
}

#[test]
fn test_bcrypt_as_str() {
    let alphabet = Alphabet::BCRYPT;
    let result = alphabet.as_str();
}

#[test]
fn test_imap_mut7_as_str() {
    let alphabet = Alphabet::IMAP_MUTF7;
    let result = alphabet.as_str();
}

#[test]
fn test_bin_hex_as_str() {
    let alphabet = Alphabet::BIN_HEX;
    let result = alphabet.as_str();
}

