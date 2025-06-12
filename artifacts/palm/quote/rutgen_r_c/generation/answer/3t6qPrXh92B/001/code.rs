// Answer 0

#[test]
fn test_fmt_with_r_prefix() {
    use proc_macro2::Ident;
    use core::fmt::Formatter;

    let id = Ident::new("r#example");
    let mut formatter = Formatter::default();
    let result = id.fmt(&mut formatter);

    assert!(result.is_ok()); // should not panic and should return Ok
}

#[test]
fn test_fmt_without_r_prefix() {
    use proc_macro2::Ident;
    use core::fmt::Formatter;

    let id = Ident::new("example");
    let mut formatter = Formatter::default();
    let result = id.fmt(&mut formatter);

    assert!(result.is_ok()); // should not panic and should return Ok
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_fmt_with_empty_id() {
    use proc_macro2::Ident;
    use core::fmt::Formatter;

    let id = Ident::new(""); // empty identifier
    let mut formatter = Formatter::default();
    let _ = id.fmt(&mut formatter); // expecting this to panic
}

