// Answer 0

#[test]
fn test_fmt_with_newline() {
    struct ByteVec<'a>(&'a [u8]);
    
    let bytes = ByteVec(&[b'\n']);
    let mut output = Vec::new();
    let result = std::fmt::write(&mut output, |f| bytes.fmt(f));
    
    assert!(result.is_ok());
    assert_eq!(output, b"b\"\\n\"");
}

#[test]
fn test_fmt_with_carriage_return() {
    struct ByteVec<'a>(&'a [u8]);
    
    let bytes = ByteVec(&[b'\r']);
    let mut output = Vec::new();
    let result = std::fmt::write(&mut output, |f| bytes.fmt(f));
    
    assert!(result.is_ok());
    assert_eq!(output, b"b\"\\r\"");
}

#[test]
fn test_fmt_with_tab() {
    struct ByteVec<'a>(&'a [u8]);
    
    let bytes = ByteVec(&[b'\t']);
    let mut output = Vec::new();
    let result = std::fmt::write(&mut output, |f| bytes.fmt(f));
    
    assert!(result.is_ok());
    assert_eq!(output, b"b\"\\t\"");
}

#[test]
fn test_fmt_with_printable_character() {
    struct ByteVec<'a>(&'a [u8]);
    
    let bytes = ByteVec(&[b'a']);
    let mut output = Vec::new();
    let result = std::fmt::write(&mut output, |f| bytes.fmt(f));
    
    assert!(result.is_ok());
    assert_eq!(output, b"b\"a\"");
}

#[test]
fn test_fmt_with_non_printable_character() {
    struct ByteVec<'a>(&'a [u8]);
    
    let bytes = ByteVec(&[b'\x01']);
    let mut output = Vec::new();
    let result = std::fmt::write(&mut output, |f| bytes.fmt(f));
    
    assert!(result.is_ok());
    assert_eq!(output, b"b\"\\x01\"");
}

#[test]
#[should_panic]
fn test_fmt_with_escape_sequence() {
    struct ByteVec<'a>(&'a [u8]);
    
    let bytes = ByteVec(&[b'\\']);
    let mut output = Vec::new();
    let _ = std::fmt::write(&mut output, |f| bytes.fmt(f));
}

