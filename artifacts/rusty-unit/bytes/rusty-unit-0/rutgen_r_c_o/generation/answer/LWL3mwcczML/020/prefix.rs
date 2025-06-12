// Answer 0

#[test]
fn test_debug_bytesref_with_newline() {
    let data = BytesRef(&[b'\n']);
    let mut formatter = Formatter::new();
    data.fmt(&mut formatter).unwrap();
}

#[test]
fn test_debug_bytesref_with_carriage_return() {
    let data = BytesRef(&[b'\r']);
    let mut formatter = Formatter::new();
    data.fmt(&mut formatter).unwrap();
}

#[test]
fn test_debug_bytesref_with_tab() {
    let data = BytesRef(&[b'\t']);
    let mut formatter = Formatter::new();
    data.fmt(&mut formatter).unwrap();
}

#[test]
fn test_debug_bytesref_with_backslash() {
    let data = BytesRef(&[b'\\']);
    let mut formatter = Formatter::new();
    data.fmt(&mut formatter).unwrap();
}

#[test]
fn test_debug_bytesref_with_double_quote() {
    let data = BytesRef(&[b'"']);
    let mut formatter = Formatter::new();
    data.fmt(&mut formatter).unwrap();
}

#[test]
fn test_debug_bytesref_with_null() {
    let data = BytesRef(&[b'\0']);
    let mut formatter = Formatter::new();
    data.fmt(&mut formatter).unwrap();
}

#[test]
fn test_debug_bytesref_with_printable_ascii() {
    let data = BytesRef(&[b'A', b'B', b'C']);
    let mut formatter = Formatter::new();
    data.fmt(&mut formatter).unwrap();
}

#[test]
fn test_debug_bytesref_with_non_printable_ascii() {
    let data = BytesRef(&[0x1f, 0x80, 0xff]);
    let mut formatter = Formatter::new();
    data.fmt(&mut formatter).unwrap();
}

