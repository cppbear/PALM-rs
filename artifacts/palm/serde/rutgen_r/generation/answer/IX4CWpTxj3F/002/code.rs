// Answer 0

#[test]
fn test_fmt_struct_variant() {
    use std::fmt;

    struct StructVariant;

    impl fmt::Debug for StructVariant {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("struct variant")
        }
    }

    let variant = StructVariant;
    let mut output = String::new();
    {
        let mut formatter = fmt::Formatter::new(&mut output);
        variant.fmt(&mut formatter).unwrap();
    }
    assert_eq!(output, "struct variant");
}

