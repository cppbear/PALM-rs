// Answer 0

#[test]
fn test_escape_bytes_empty() {
    let input: &[u8] = &[];
    let expected = String::new();
    let result = escape_bytes(input);
    assert_eq!(result, expected);
}

#[test]
fn test_escape_bytes_single_byte() {
    fn escape_byte(b: u8) -> String {
        match b {
            b'\\' => String::from(r"\"),
            b'\n' => String::from(r"\n"),
            b'\t' => String::from(r"\t"),
            _ => (b as char).to_string(),
        }
    }

    let input: &[u8] = &[b'a'];
    let expected = String::from("a");
    let result = escape_bytes(input);
    assert_eq!(result, expected);
}

#[test]
fn test_escape_bytes_escape_characters() {
    fn escape_byte(b: u8) -> String {
        match b {
            b'\\' => String::from(r"\"),
            b'\n' => String::from(r"\n"),
            b'\t' => String::from(r"\t"),
            _ => (b as char).to_string(),
        }
    }

    let input: &[u8] = &[b'a', b'\\', b'\n', b'\t', b'b'];
    let expected = String::from("a\\\n\tb");
    let result = escape_bytes(input);
    assert_eq!(result, expected);
}

#[test]
fn test_escape_bytes_all_escape_characters() {
    fn escape_byte(b: u8) -> String {
        match b {
            b'\\' => String::from(r"\"),
            b'\n' => String::from(r"\n"),
            b'\t' => String::from(r"\t"),
            _ => (b as char).to_string(),
        }
    }

    let input: &[u8] = &[b'\\', b'\n', b'\t'];
    let expected = String::from("\\\n\t");
    let result = escape_bytes(input);
    assert_eq!(result, expected);
}

