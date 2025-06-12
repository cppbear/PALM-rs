// Answer 0

#[test]
fn test_fmt_with_newline() {
    let data = BytesRef(b"\n");
    let mut buffer = Formatter::new(&mut Vec::new());
    let _ = data.fmt(&mut buffer);
}

#[test]
fn test_fmt_with_carriage_return() {
    let data = BytesRef(b"\r");
    let mut buffer = Formatter::new(&mut Vec::new());
    let _ = data.fmt(&mut buffer);
}

#[test]
fn test_fmt_with_tab() {
    let data = BytesRef(b"\t");
    let mut buffer = Formatter::new(&mut Vec::new());
    let _ = data.fmt(&mut buffer);
}

#[test]
fn test_fmt_with_ascii_printable() {
    let data = BytesRef(b"Hello, World!");
    let mut buffer = Formatter::new(&mut Vec::new());
    let _ = data.fmt(&mut buffer);
}

#[test]
fn test_fmt_with_escape_sequence() {
    let data = BytesRef(b"\\hello\\");
    let mut buffer = Formatter::new(&mut Vec::new());
    let _ = data.fmt(&mut buffer);
}

#[test]
fn test_fmt_with_non_printable_low() {
    let data = BytesRef(b"\x01\x02\x03\x1f");
    let mut buffer = Formatter::new(&mut Vec::new());
    let _ = data.fmt(&mut buffer);
}

#[test]
fn test_fmt_with_non_printable_high() {
    let data = BytesRef(b"\x80\x81\xff");
    let mut buffer = Formatter::new(&mut Vec::new());
    let _ = data.fmt(&mut buffer);
}

#[test]
#[should_panic]
fn test_fmt_should_panic_on_invalid_write() {
    let data = BytesRef(b"\\");
    let mut buffer = Formatter::new(&mut Vec::new());
    let _ = data.fmt(&mut buffer);
}

