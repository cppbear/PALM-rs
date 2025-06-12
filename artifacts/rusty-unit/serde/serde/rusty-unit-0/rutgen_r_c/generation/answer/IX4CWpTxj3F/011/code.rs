// Answer 0

#[test]
fn test_fmt_unit() {
    use std::fmt;

    struct FormatterMock;

    impl fmt::Write for FormatterMock {
        fn write_str(&mut self, _s: &str) -> fmt::Result {
            Ok(())
        }
    }

    let unit_value = Unexpected::Unit;
    let mut formatter = FormatterMock;

    assert!(unit_value.fmt(&mut formatter).is_ok());
}

#[test]
fn test_fmt_unit_variant() {
    use std::fmt;

    struct FormatterMock;

    impl fmt::Write for FormatterMock {
        fn write_str(&mut self, _s: &str) -> fmt::Result {
            Ok(())
        }
    }

    let unit_variant = Unexpected::UnitVariant;
    let mut formatter = FormatterMock;

    assert!(unit_variant.fmt(&mut formatter).is_ok());
}

