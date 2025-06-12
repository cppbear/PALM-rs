// Answer 0

#[test]
fn test_ignore_str_with_consecutive_escaped_quotes() {
    let mut scratch = Vec::new();
    let mut reader = SliceRead::new(&[b'\\', b'\\', b'"']);
    reader.ignore_str();
}

#[test]
fn test_ignore_str_with_single_escape() {
    let mut scratch = Vec::new();
    let mut reader = SliceRead::new(&[b'\\', b'"']);
    reader.ignore_str();
}

#[test]
fn test_ignore_str_with_control_character() {
    let mut scratch = Vec::new();
    let mut reader = SliceRead::new(&[b'\\', b'\x01', b'"']);
    reader.ignore_str();
}

#[test]
#[should_panic]
fn test_ignore_str_with_eof() {
    let mut scratch = Vec::new();
    let mut reader = SliceRead::new(&[b'\\', b'\\']);
    reader.ignore_str();
}

#[test]
#[should_panic]
fn test_ignore_str_with_invalid_escape() {
    let mut scratch = Vec::new();
    let mut reader = SliceRead::new(&[b'\\', b'\x01']); 
    reader.ignore_str();
}

