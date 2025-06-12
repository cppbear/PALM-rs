// Answer 0

#[test]
fn test_deserialize_option_none() {
    struct VisitorNone;

    impl<'de> Visitor<'de> for VisitorNone {
        type Value = Option<()>;

        fn visit_none(self) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(None)
        }

        fn visit_some<V>(self, _: V) -> Result<Self::Value, Box<dyn std::error::Error>> {
            panic!("should not visit some()");
        }

        fn visit_unit(self) -> Result<Self::Value, Box<dyn std::error::Error>> {
            panic!("should not visit unit()");
        }
    }

    let content = Content::None;
    let deserializer = ContentDeserializer::new(content);
    let result: Option<()> = deserializer.deserialize_option(VisitorNone).unwrap();
    assert_eq!(result, None);
}

#[test]
fn test_deserialize_option_some() {
    struct VisitorSome;

    impl<'de> Visitor<'de> for VisitorSome {
        type Value = Option<u8>;

        fn visit_none(self) -> Result<Self::Value, Box<dyn std::error::Error>> {
            panic!("should not visit none()");
        }

        fn visit_some<V>(self, _: V) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(Some(42))
        }

        fn visit_unit(self) -> Result<Self::Value, Box<dyn std::error::Error>> {
            panic!("should not visit unit()");
        }
    }

    let content = Content::Some(Box::new(Content::U8(42)));
    let deserializer = ContentDeserializer::new(content);
    let result: Option<u8> = deserializer.deserialize_option(VisitorSome).unwrap();
    assert_eq!(result, Some(42));
}

#[test]
fn test_deserialize_option_unit() {
    struct VisitorUnit;

    impl<'de> Visitor<'de> for VisitorUnit {
        type Value = Option<()>;

        fn visit_none(self) -> Result<Self::Value, Box<dyn std::error::Error>> {
            panic!("should not visit none()");
        }

        fn visit_some<V>(self, _: V) -> Result<Self::Value, Box<dyn std::error::Error>> {
            panic!("should not visit some()");
        }

        fn visit_unit(self) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(Some(()))
        }
    }

    let content = Content::Unit;
    let deserializer = ContentDeserializer::new(content);
    let result: Option<()> = deserializer.deserialize_option(VisitorUnit).unwrap();
    assert_eq!(result, Some(()));
}

