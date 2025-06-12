// Answer 0

#[test]
fn test_parse_escape_valid_double_quote() {
    let mut scratch = Vec::new();
    let mut reader = &b"\""[..];
    let validate = true;
    parse_escape(&mut reader, validate, &mut scratch);
}

#[test]
fn test_parse_escape_valid_backslash() {
    let mut scratch = Vec::new();
    let mut reader = &b"\\"[..];
    let validate = true;
    parse_escape(&mut reader, validate, &mut scratch);
}

#[test]
fn test_parse_escape_valid_forward_slash() {
    let mut scratch = Vec::new();
    let mut reader = &b"/"[..];
    let validate = true;
    parse_escape(&mut reader, validate, &mut scratch);
}

#[test]
fn test_parse_escape_valid_backspace() {
    let mut scratch = Vec::new();
    let mut reader = &b"b"[..];
    let validate = true;
    parse_escape(&mut reader, validate, &mut scratch);
}

#[test]
fn test_parse_escape_valid_form_feed() {
    let mut scratch = Vec::new();
    let mut reader = &b"f"[..];
    let validate = true;
    parse_escape(&mut reader, validate, &mut scratch);
}

#[test]
fn test_parse_escape_valid_newline() {
    let mut scratch = Vec::new();
    let mut reader = &b"n"[..];
    let validate = true;
    parse_escape(&mut reader, validate, &mut scratch);
}

#[test]
fn test_parse_escape_valid_carriage_return() {
    let mut scratch = Vec::new();
    let mut reader = &b"r"[..];
    let validate = true;
    parse_escape(&mut reader, validate, &mut scratch);
}

#[test]
fn test_parse_escape_valid_horizontal_tab() {
    let mut scratch = Vec::new();
    let mut reader = &b"t"[..];
    let validate = true;
    parse_escape(&mut reader, validate, &mut scratch);
}

#[test]
fn test_parse_escape_invalid_escape() {
    let mut scratch = Vec::new();
    let mut reader = &b"x"[..];
    let validate = true;
    parse_escape(&mut reader, validate, &mut scratch);
}

#[test]
fn test_parse_escape_valid_unicode_escape() {
    let mut scratch = Vec::new();
    let mut reader = &b"u"[..];
    let validate = true;
    parse_escape(&mut reader, validate, &mut scratch);
}

