// Answer 0

#[test]
fn test_allow_invalid_utf8_true() {
    let mut builder = ParserBuilder::new();
    builder.allow_invalid_utf8(true);
}

#[test]
fn test_allow_invalid_utf8_false() {
    let mut builder = ParserBuilder::new();
    builder.allow_invalid_utf8(false);
}

#[test]
fn test_allow_invalid_utf8_multiple_true() {
    let mut builder = ParserBuilder::new();
    builder.allow_invalid_utf8(true)
           .allow_invalid_utf8(true);
}

#[test]
fn test_allow_invalid_utf8_multiple_false() {
    let mut builder = ParserBuilder::new();
    builder.allow_invalid_utf8(false)
           .allow_invalid_utf8(false);
}

#[test]
fn test_allow_invalid_utf8_chain() {
    let mut builder = ParserBuilder::new();
    builder.allow_invalid_utf8(true)
           .allow_invalid_utf8(false);
}

