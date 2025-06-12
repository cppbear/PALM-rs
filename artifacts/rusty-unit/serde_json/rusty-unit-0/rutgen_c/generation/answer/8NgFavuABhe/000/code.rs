// Answer 0

#[test]
fn test_visit_str_map() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = KeyClass;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string")
        }

        fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            match s {
                #[cfg(feature = "arbitrary_precision")]
                crate::number::TOKEN => Ok(KeyClass::Number),
                #[cfg(feature = "raw_value")]
                crate::raw::TOKEN => Ok(KeyClass::RawValue),
                _ => Ok(KeyClass::Map(s.to_owned())),
            }
        }
    }

    let visitor = TestVisitor;
    let result = visitor.visit_str("some_key").unwrap();
    match result {
        KeyClass::Map(ref key) => assert_eq!(key, "some_key"),
        _ => panic!("Expected KeyClass::Map"),
    }
}

#[test]
#[cfg(feature = "arbitrary_precision")]
fn test_visit_str_number() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = KeyClass;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string")
        }

        fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            match s {
                #[cfg(feature = "arbitrary_precision")]
                crate::number::TOKEN => Ok(KeyClass::Number),
                #[cfg(feature = "raw_value")]
                crate::raw::TOKEN => Ok(KeyClass::RawValue),
                _ => Ok(KeyClass::Map(s.to_owned())),
            }
        }
    }

    let visitor = TestVisitor;
    let result = visitor.visit_str(crate::number::TOKEN).unwrap();
    match result {
        KeyClass::Number => {}
        _ => panic!("Expected KeyClass::Number"),
    }
}

#[test]
#[cfg(feature = "raw_value")]
fn test_visit_str_raw_value() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = KeyClass;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string")
        }

        fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            match s {
                #[cfg(feature = "arbitrary_precision")]
                crate::number::TOKEN => Ok(KeyClass::Number),
                #[cfg(feature = "raw_value")]
                crate::raw::TOKEN => Ok(KeyClass::RawValue),
                _ => Ok(KeyClass::Map(s.to_owned())),
            }
        }
    }

    let visitor = TestVisitor;
    let result = visitor.visit_str(crate::raw::TOKEN).unwrap();
    match result {
        KeyClass::RawValue => {}
        _ => panic!("Expected KeyClass::RawValue"),
    }
}

