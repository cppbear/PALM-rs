// Answer 0

#[test]
fn test_new_with_valid_alphabet() {
    let result = Alphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/");
}

#[test]
fn test_new_with_minimum_ascii() {
    let result = Alphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYYZabcdefghijklmnopqrstuvwxy0123456789+/");
}

#[test]
fn test_new_with_maximum_ascii() {
    let result = Alphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/");
}

