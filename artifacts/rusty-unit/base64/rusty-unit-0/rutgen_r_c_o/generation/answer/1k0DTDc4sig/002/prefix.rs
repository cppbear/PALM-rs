// Answer 0

#[test]
fn test_from_str_unchecked() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let _ = Alphabet::from_str_unchecked(alphabet);
}

#[test]
fn test_from_str_unchecked_with_alphabet_size() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
    let _ = Alphabet::from_str_unchecked(alphabet);
}

#[test]
fn test_from_str_unchecked_with_crypt() {
    let alphabet = "./0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let _ = Alphabet::from_str_unchecked(alphabet);
}

#[test]
fn test_from_str_unchecked_with_bcrypt() {
    let alphabet = "./ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let _ = Alphabet::from_str_unchecked(alphabet);
}

#[test]
fn test_from_str_unchecked_with_imap_mut7() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+,";
    let _ = Alphabet::from_str_unchecked(alphabet);
}

#[test]
fn test_from_str_unchecked_with_bin_hex() {
    let alphabet = "!\"#$%&'()*+,-012345689@ABCDEFGHIJKLMNPQRSTUVXYZ[`abcdefhijklmpqr";
    let _ = Alphabet::from_str_unchecked(alphabet);
}

#[test]
#[should_panic]
fn test_from_str_unchecked_panic_due_to_too_short() {
    let alphabet = "ABC"; // Less than 64 characters
    let _ = Alphabet::from_str_unchecked(alphabet);
}

#[test]
#[should_panic]
fn test_from_str_unchecked_panic_due_to_too_long() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/123"; // More than 64 characters
    let _ = Alphabet::from_str_unchecked(alphabet);
}

#[test]
#[should_panic]
fn test_from_str_unchecked_panic_due_to_invalid_bytes() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWX\0YZabcdefghijklmnopqrstuvwxyz0123456789+/"; // Contains null byte
    let _ = Alphabet::from_str_unchecked(alphabet);
}

