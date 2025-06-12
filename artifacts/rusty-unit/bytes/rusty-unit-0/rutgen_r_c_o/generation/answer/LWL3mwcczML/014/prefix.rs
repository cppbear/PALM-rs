// Answer 0

#[test]
fn test_fmt_with_newline() {
    let data = BytesRef(&[b'\n']);
    let mut formatter = Formatter::new();
    data.fmt(&mut formatter).unwrap();
}

#[test]
fn test_fmt_with_carriage_return() {
    let data = BytesRef(&[b'\r']);
    let mut formatter = Formatter::new();
    data.fmt(&mut formatter).unwrap();
}

#[test]
fn test_fmt_with_tab() {
    let data = BytesRef(&[b'\t']);
    let mut formatter = Formatter::new();
    data.fmt(&mut formatter).unwrap();
}

#[test]
fn test_fmt_with_backslash() {
    let data = BytesRef(&[b'\\']);
    let mut formatter = Formatter::new();
    data.fmt(&mut formatter).unwrap();
}

#[test]
fn test_fmt_with_non_printable() {
    let data = BytesRef(&[0x01]);
    let mut formatter = Formatter::new();
    data.fmt(&mut formatter).unwrap();
}

#[test]
fn test_fmt_with_control_character() {
    let data = BytesRef(&[0x1F]);
    let mut formatter = Formatter::new();
    data.fmt(&mut formatter).unwrap();
}

