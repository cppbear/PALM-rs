// Answer 0

#[test]
fn test_new_alphabet_with_duplicate_character() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/A"; // 'A' is duplicated
    let result = Alphabet::new(alphabet);
}

#[test]
fn test_new_alphabet_with_duplicate_character_non_printable() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYabcdefghiJKLMNOpqrstuvwxyz0123456789+/"; // 'O' is duplicated
    let result = Alphabet::new(alphabet);
}

#[test]
fn test_new_alphabet_with_duplicate_character_edge_case() {
    let alphabet = "\x20\x21\x22\x23\x24\x25\x26\x27\x28\x29\x2A\x2B\x30\x31\x32\x33\x34\x35\x36\x37\x38\x39\x3A\x3B\x3C\x3D\x3E\x3F\x40\x41\x42\x43\x44\x45\x46\x47\x48\x49\x4A\x4B\x4C\x4D\x4E\x4F\x50\x51\x52\x53\x54\x55\x56\x57\x58\x59\x5A\x5B\x5C\x5D\x5E\x5F"; // '\x20' to '\x5F' (64 characters with duplicates)
    let result = Alphabet::new(alphabet);
}

