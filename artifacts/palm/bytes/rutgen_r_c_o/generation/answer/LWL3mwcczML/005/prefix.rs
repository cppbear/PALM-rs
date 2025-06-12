// Answer 0

#[test]
fn test_fmt_newline() {
    let bytes_ref = BytesRef(b"\n");
    let mut fmt = Formatter::new();
    let _ = bytes_ref.fmt(&mut fmt);
}

#[test]
fn test_fmt_multiple_newlines() {
    let bytes_ref = BytesRef(b"\n\n\n");
    let mut fmt = Formatter::new();
    let _ = bytes_ref.fmt(&mut fmt);
}

#[test]
fn test_fmt_non_printable() {
    let bytes_ref = BytesRef(b"\x00\x1f");
    let mut fmt = Formatter::new();
    let _ = bytes_ref.fmt(&mut fmt);
}

#[test]
fn test_fmt_above_control_char() {
    let bytes_ref = BytesRef(b"\x80\xff");
    let mut fmt = Formatter::new();
    let _ = bytes_ref.fmt(&mut fmt);
}

#[test]
fn test_fmt_mixed_content() {
    let bytes_ref = BytesRef(b"\n\x00\x1f\x80");
    let mut fmt = Formatter::new();
    let _ = bytes_ref.fmt(&mut fmt);
}

#[test]
#[should_panic]
fn test_fmt_r_not_written() {
    let bytes_ref = BytesRef(b"\r");
    let mut fmt = Formatter::new();
    let _ = bytes_ref.fmt(&mut fmt);
}

