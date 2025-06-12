// Answer 0

#[test]
fn test_fmt_unit_variant() {
    use serde::de::Unexpected;
    use std::fmt;

    struct UnitVariant;

    impl fmt::Display for UnitVariant {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            Unexpected::UnitVariant.fmt(formatter)
        }
    }

    let unit_variant = UnitVariant;
    let mut output = String::new();
    let result = write!(&mut output, "{}", unit_variant);
    assert!(result.is_ok());
    assert_eq!(output, "unit variant");
}

#[test]
fn test_fmt_another_unit_variant() {
    use serde::de::Unexpected;
    use std::fmt;

    struct AnotherUnitVariant;

    impl fmt::Display for AnotherUnitVariant {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            Unexpected::UnitVariant.fmt(formatter)
        }
    }

    let another_unit_variant = AnotherUnitVariant;
    let mut output = String::new();
    let result = write!(&mut output, "{}", another_unit_variant);
    assert!(result.is_ok());
    assert_eq!(output, "unit variant");
}

