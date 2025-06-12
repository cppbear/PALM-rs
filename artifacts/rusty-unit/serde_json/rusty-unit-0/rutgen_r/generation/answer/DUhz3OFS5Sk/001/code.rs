// Answer 0

#[test]
fn test_struct_variant_invalid_type() {
    struct TestVisitor;

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = ();

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a struct variant")
        }

        fn visit_unit_variant<E>(self) -> Result<Self::Value, E> 
        where 
            E: serde::de::Error
        {
            Err(E::custom("cannot visit unit variant"))
        }
    }

    let fields: &'static [&'static str] = &["field1", "field2"];
    
    let result: Result<(), serde::de::Error> = struct_variant(fields, TestVisitor);
    
    match result {
        Err(serde::de::Error::InvalidType { .. }) => {}
        _ => panic!("Expected Err(de::Error::invalid_type(...)), but got {:?}", result),
    }
}

#[test]
fn test_struct_variant_with_empty_fields() {
    struct TestVisitor;

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = ();

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a struct variant")
        }

        fn visit_unit_variant<E>(self) -> Result<Self::Value, E> 
        where 
            E: serde::de::Error
        {
            Err(E::custom("cannot visit unit variant"))
        }
    }

    let fields: &'static [&'static str] = &[];

    let result: Result<(), serde::de::Error> = struct_variant(fields, TestVisitor);

    match result {
        Err(serde::de::Error::InvalidType { .. }) => {}
        _ => panic!("Expected Err(de::Error::invalid_type(...)), but got {:?}", result),
    }
}

