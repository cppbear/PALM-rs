// Answer 0

#[test]
fn test_parse_escape_double_quote() {
    let mut scratch = Vec::new();
    let input = vec![b'\\', b'"'];
    let mut read = &input[..];
    let validate = true;
    let _ = parse_escape(&mut read, validate, &mut scratch);
}

#[test]
fn test_parse_escape_backslash() {
    let mut scratch = Vec::new();
    let input = vec![b'\\', b'\\'];
    let mut read = &input[..];
    let validate = true;
    let _ = parse_escape(&mut read, validate, &mut scratch);
}

#[test]
fn test_parse_escape_backspace() {
    let mut scratch = Vec::new();
    let input = vec![b'\\', b'b'];
    let mut read = &input[..];
    let validate = true;
    let _ = parse_escape(&mut read, validate, &mut scratch);
}

#[test]
fn test_parse_escape_form_feed() {
    let mut scratch = Vec::new();
    let input = vec![b'\\', b'f'];
    let mut read = &input[..];
    let validate = true;
    let _ = parse_escape(&mut read, validate, &mut scratch);
}

#[test]
fn test_parse_escape_newline() {
    let mut scratch = Vec::new();
    let input = vec![b'\\', b'n'];
    let mut read = &input[..];
    let validate = true;
    let _ = parse_escape(&mut read, validate, &mut scratch);
}

#[test]
fn test_parse_escape_carriage_return() {
    let mut scratch = Vec::new();
    let input = vec![b'\\', b'r'];
    let mut read = &input[..];
    let validate = true;
    let _ = parse_escape(&mut read, validate, &mut scratch);
}

#[test]
fn test_parse_escape_tab() {
    let mut scratch = Vec::new();
    let input = vec![b'\\', b't'];
    let mut read = &input[..];
    let validate = true;
    let _ = parse_escape(&mut read, validate, &mut scratch);
}

#[test]
fn test_parse_escape_forward_slash() {
    let mut scratch = Vec::new();
    let input = vec![b'\\', b'/'];
    let mut read = &input[..];
    let validate = true;
    let _ = parse_escape(&mut read, validate, &mut scratch);
}

#[test]
fn test_parse_escape_unicode() {
    let mut scratch = Vec::new();
    let input = vec![b'\\', b'u', b'1', b'2', b'3', b'4'];
    let mut read = &input[..];
    let validate = true;
    let _ = parse_escape(&mut read, validate, &mut scratch);
}

#[test]
#[should_panic]
fn test_parse_escape_invalid_escape() {
    let mut scratch = Vec::new();
    let input = vec![b'\\', b'x'];
    let mut read = &input[..];
    let validate = true;
    let _ = parse_escape(&mut read, validate, &mut scratch);
}

