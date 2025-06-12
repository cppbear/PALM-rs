// Answer 0

#[test]
fn test_fmt_with_newline() {
    let data = BytesRef(&[b'\n']);
    let mut buffer = String::new();
    let mut formatter = Formatter::new(&mut buffer);
    data.fmt(&mut formatter).expect("Formatting failed");
}

#[test]
fn test_fmt_with_carriage_return() {
    let data = BytesRef(&[b'\r']);
    let mut buffer = String::new();
    let mut formatter = Formatter::new(&mut buffer);
    data.fmt(&mut formatter).expect("Formatting failed");
}

#[test]
fn test_fmt_with_tab() {
    let data = BytesRef(&[b'\t']);
    let mut buffer = String::new();
    let mut formatter = Formatter::new(&mut buffer);
    data.fmt(&mut formatter).expect("Formatting failed");
}

#[test]
fn test_fmt_with_backslash() {
    let data = BytesRef(&[b'\\']);
    let mut buffer = String::new();
    let mut formatter = Formatter::new(&mut buffer);
    data.fmt(&mut formatter).expect("Formatting failed");
}

#[test]
fn test_fmt_with_double_quote() {
    let data = BytesRef(&[b'"']);
    let mut buffer = String::new();
    let mut formatter = Formatter::new(&mut buffer);
    data.fmt(&mut formatter).expect("Formatting failed");
}

#[test]
fn test_fmt_with_null_byte() {
    let data = BytesRef(&[b'\0']);
    let mut buffer = String::new();
    let mut formatter = Formatter::new(&mut buffer);
    data.fmt(&mut formatter).expect("Formatting failed");
}

#[test]
fn test_fmt_with_printable_ascii() {
    let data = BytesRef(&[b'A', b'B', b'C']);
    let mut buffer = String::new();
    let mut formatter = Formatter::new(&mut buffer);
    data.fmt(&mut formatter).expect("Formatting failed");
}

#[test]
fn test_fmt_with_non_printable_bytes() {
    let data = BytesRef(&[0x01, 0x02, 0x03, 0xFF]);
    let mut buffer = String::new();
    let mut formatter = Formatter::new(&mut buffer);
    data.fmt(&mut formatter).expect("Formatting failed");
}

