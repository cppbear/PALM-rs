// Answer 0

#[test]
fn test_fmt_debug_bytes_ref_non_empty() {
    let bytes = BytesRef(b"abc\x00def");
    let mut output = Vec::new();
    let formatter = &mut Formatter::new(&mut output);
    bytes.fmt(formatter).unwrap();
}

#[test]
fn test_fmt_debug_bytes_ref_empty() {
    let bytes = BytesRef(b"");
    let mut output = Vec::new();
    let formatter = &mut Formatter::new(&mut output);
    bytes.fmt(formatter).unwrap();
}

#[test]
fn test_fmt_debug_bytes_ref_with_newline() {
    let bytes = BytesRef(b"abc\nxyz");
    let mut output = Vec::new();
    let formatter = &mut Formatter::new(&mut output);
    bytes.fmt(formatter).unwrap();
}

#[test]
fn test_fmt_debug_bytes_ref_with_special_characters() {
    let bytes = BytesRef(b"abc\r\t\"\\" );
    let mut output = Vec::new();
    let formatter = &mut Formatter::new(&mut output);
    bytes.fmt(formatter).unwrap();
}

#[test]
fn test_fmt_debug_bytes_ref_with_control_characters() {
    let bytes = BytesRef(b"abc\x01\x02\x03");
    let mut output = Vec::new();
    let formatter = &mut Formatter::new(&mut output);
    bytes.fmt(formatter).unwrap();
}

