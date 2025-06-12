// Answer 0

#[derive(Debug)]
struct MockEnumAccess {
    is_variant_ok: bool,
    is_unit_variant_ok: bool,
}

impl<'de> serde::de::EnumAccess<'de> for MockEnumAccess {
    type Error = serde::de::value::Error;
    type Variant = MockVariantAccess;

    fn variant(self) -> Result<(String, Self::Variant), Self::Error> {
        if self.is_variant_ok {
            Ok(("mock_variant".to_string(), MockVariantAccess { is_ok: self.is_unit_variant_ok }))
        } else {
            Err(serde::de::value::Error::custom("Variant error"))
        }
    }
}

#[derive(Debug)]
struct MockVariantAccess {
    is_ok: bool,
}

impl<'de> serde::de::VariantAccess<'de> for MockVariantAccess {
    type Error = serde::de::value::Error;

    fn unit_variant(self) -> Result<(), Self::Error> {
        if self.is_ok {
            Ok(())
        } else {
            Err(serde::de::value::Error::custom("Unit variant error"))
        }
    }
}

#[test]
fn test_visit_enum_variant_ok_unit_variant_ok() {
    let mock_access = MockEnumAccess {
        is_variant_ok: true,
        is_unit_variant_ok: true,
    };
    let result = visit_enum(mock_access);
    assert_eq!(result, Ok("mock_variant".to_string()));
}

#[test]
fn test_visit_enum_variant_ok_unit_variant_err() {
    let mock_access = MockEnumAccess {
        is_variant_ok: true,
        is_unit_variant_ok: false,
    };
    let result = visit_enum(mock_access);
    assert!(result.is_err());
}

#[test]
fn test_visit_enum_variant_err() {
    let mock_access = MockEnumAccess {
        is_variant_ok: false,
        is_unit_variant_ok: true,
    };
    let result = visit_enum(mock_access);
    assert!(result.is_err());
}

