// Answer 0

#[test]
fn test_from_bytes_empty_input() {
    let input: &[u8] = b"";
    let result = HdrName::from_bytes(input, |hdr| hdr);
}

#[test]
fn test_from_bytes_valid_single_char() {
    let input: &[u8] = b"A";
    let result = HdrName::from_bytes(input, |hdr| hdr);
}

#[test]
fn test_from_bytes_valid_max_length() {
    let input: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let result = HdrName::from_bytes(input, |hdr| hdr);
}

#[test]
fn test_from_bytes_valid_non_standard() {
    let input: &[u8] = b"custom-header";
    let result = HdrName::from_bytes(input, |hdr| hdr);
}

#[test]
fn test_from_bytes_overflow() {
    let input: &[u8] = b"A".repeat(SCRATCH_BUF_OVERFLOW);
    let result = HdrName::from_bytes(input, |hdr| hdr);
}

#[test]
fn test_from_bytes_invalid_character() {
    let input: &[u8] = b"Invalid\x00Header";
    let result = HdrName::from_bytes(input, |hdr| hdr);
}

#[test]
fn test_from_bytes_invalid_length() {
    let input: &[u8] = &[0; 65]; // Invalid because length > 63
    let result = HdrName::from_bytes(input, |hdr| hdr);
}

#[test]
fn test_from_bytes_invalid_forbidden_characters() {
    let input: &[u8] = b"header\x01header"; // Invalid character range
    let result = HdrName::from_bytes(input, |hdr| hdr);
}

