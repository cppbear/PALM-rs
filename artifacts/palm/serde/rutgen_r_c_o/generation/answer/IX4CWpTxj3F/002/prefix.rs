// Answer 0

#[test]
fn test_fmt_struct_variant() {
    let variant = Unexpected::StructVariant;
    let mut formatter = std::fmt::Formatter::new();
    variant.fmt(&mut formatter);
}

#[test]
fn test_fmt_another_struct_variant() {
    let variant = Unexpected::StructVariant;
    let mut formatter = std::fmt::Formatter::new();
    variant.fmt(&mut formatter);
}

