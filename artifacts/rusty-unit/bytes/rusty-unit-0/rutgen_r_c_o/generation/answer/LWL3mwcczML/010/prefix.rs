// Answer 0

#[test]
fn test_bytesref_debug_normal_characters() {
    let bytes_ref = BytesRef(&[b'A', b'B', b'C']);
    let mut output = vec![];
    {
        let formatter = &mut Formatter::new(&mut output);
        let _ = bytes_ref.fmt(formatter);
    }
}

#[test]
fn test_bytesref_debug_escape_characters() {
    let bytes_ref = BytesRef(&[b'\\', b'"', b'\n', b'\r']);
    let mut output = vec![];
    {
        let formatter = &mut Formatter::new(&mut output);
        let _ = bytes_ref.fmt(formatter);
    }
}

#[test]
fn test_bytesref_debug_non_printable_characters() {
    let bytes_ref = BytesRef(&[b'\0', 0x01, 0x7F, 0x80]);
    let mut output = vec![];
    {
        let formatter = &mut Formatter::new(&mut output);
        let _ = bytes_ref.fmt(formatter);
    }
}

#[test]
fn test_bytesref_debug_printable_range() {
    let bytes_ref = BytesRef(&[b' ', b'!', b'~']);
    let mut output = vec![];
    {
        let formatter = &mut Formatter::new(&mut output);
        let _ = bytes_ref.fmt(formatter);
    }
}

#[test]
fn test_bytesref_debug_mixed_characters() {
    let bytes_ref = BytesRef(&[b'A', b'\n', b'!', b'\0', b'\\', b'Z']);
    let mut output = vec![];
    {
        let formatter = &mut Formatter::new(&mut output);
        let _ = bytes_ref.fmt(formatter);
    }
}

