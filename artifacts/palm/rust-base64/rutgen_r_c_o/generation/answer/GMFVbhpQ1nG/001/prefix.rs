// Answer 0

#[test]
fn test_new_invalid_length_below() {
    let result = Alphabet::new("ABCDEF"); // 6 characters, less than 64
}

#[test]
fn test_new_invalid_length_above() {
    let result = Alphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/="); // 65 characters, greater than 64
}

#[test]
fn test_new_invalid_length_empty() {
    let result = Alphabet::new(""); // 0 characters, less than 64
}

#[test]
fn test_new_invalid_length_partial() {
    let result = Alphabet::new("0123456789ABCDEF"); // 16 characters, less than 64
}

#[test]
fn test_new_invalid_length_exactly_64_with_extra_space() {
    let result = Alphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+ /"); // 64 characters but contains a space
}

