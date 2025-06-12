// Answer 0

#[test]
fn test_fmt_newtype_variant() {
    let variant_input = Unexpected::NewtypeVariant;
    let mut output = String::new();
    let mut formatter = fmt::Formatter::new(&mut output);
    variant_input.fmt(&mut formatter);
}

#[test]
fn test_fmt_newtype_variant_with_altered_message() {
    let variant_input = Unexpected::Other("newtype variant");
    let mut output = String::new();
    let mut formatter = fmt::Formatter::new(&mut output);
    variant_input.fmt(&mut formatter);
}

