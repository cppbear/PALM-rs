// Answer 0

#[test]
fn test_literal_fmt_complete_non_empty_ascii() {
    let bytes = vec![b'R', b'u', b's', b't'];
    let mut literal = Literal::new(bytes);
    literal.cut = false;
    let _ = format!("{:?}", literal);
}

#[test]
fn test_literal_fmt_complete_non_empty_unicode() {
    let bytes = vec![0xE2, 0x9C, 0x94]; // Unicode checkmark
    let mut literal = Literal::new(bytes);
    literal.cut = false;
    let _ = format!("{:?}", literal);
}

#[test]
fn test_literal_fmt_complete_whitespace() {
    let bytes = vec![b' ', b'\t', b'\n'];
    let mut literal = Literal::new(bytes);
    literal.cut = false;
    let _ = format!("{:?}", literal);
}

#[test]
fn test_literal_fmt_complete_special_characters() {
    let bytes = vec![b'!', b'@', b'#', b'$'];
    let mut literal = Literal::new(bytes);
    literal.cut = false;
    let _ = format!("{:?}", literal);
}

#[test]
fn test_literal_fmt_complete_large_utf8() {
    let bytes = (0..512).map(|x| (x % 128) as u8).collect::<Vec<u8>>();
    let mut literal = Literal::new(bytes);
    literal.cut = false;
    let _ = format!("{:?}", literal);
}

