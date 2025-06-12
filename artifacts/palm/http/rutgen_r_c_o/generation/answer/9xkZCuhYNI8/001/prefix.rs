// Answer 0

#[test]
fn test_eq_ignore_ascii_case_length_1_vs_0() {
    let lower: &[u8] = b"a";
    let s: &[u8] = b"";
    eq_ignore_ascii_case(lower, s);
}

#[test]
fn test_eq_ignore_ascii_case_length_256_vs_255() {
    let lower: &[u8] = &vec![b'a'; 256];
    let s: &[u8] = &vec![b'a'; 255];
    eq_ignore_ascii_case(lower, s);
}

#[test]
fn test_eq_ignore_ascii_case_length_0_vs_1() {
    let lower: &[u8] = b"";
    let s: &[u8] = b"a";
    eq_ignore_ascii_case(lower, s);
}

#[test]
fn test_eq_ignore_ascii_case_length_8_vs_9() {
    let lower: &[u8] = b"abcdefgh";
    let s: &[u8] = b"abcdefghi";
    eq_ignore_ascii_case(lower, s);
}

#[test]
fn test_eq_ignore_ascii_case_length_10_vs_20() {
    let lower: &[u8] = b"abcdefghij";
    let s: &[u8] = b"abcdefghij1234567890";
    eq_ignore_ascii_case(lower, s);
}

#[test]
fn test_eq_ignore_ascii_case_length_15_vs_16() {
    let lower: &[u8] = b"abcdefghijklmno";
    let s: &[u8] = b"abcdefghijklmnop";
    eq_ignore_ascii_case(lower, s);
}

