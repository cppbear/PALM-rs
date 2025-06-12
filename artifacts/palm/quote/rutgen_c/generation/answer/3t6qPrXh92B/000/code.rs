// Answer 0

#[test]
fn test_fmt_ident_with_r_prefix() {
    use std::fmt::Formatter;
    use proc_macro2::Ident;

    let ident = Ident::new("r#example", Span::call_site());
    let mut output = String::new();
    let result = ident.fmt(&mut Formatter::new(&mut output));

    assert!(result.is_ok());
    assert_eq!(output, "example");
}

#[test]
fn test_fmt_ident_without_r_prefix() {
    use std::fmt::Formatter;
    use proc_macro2::Ident;

    let ident = Ident::new("example", Span::call_site());
    let mut output = String::new();
    let result = ident.fmt(&mut Formatter::new(&mut output));

    assert!(result.is_ok());
    assert_eq!(output, "example");
}

